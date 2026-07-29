use crate::core::date::NaiveHijriDate;
use crate::core::search::search_city_details;
use crate::core::storage::{db_connection, get_config_path, load_config, save_config};
use crate::core::time::{CalendarPrayerTimes, DayPrayerTimes, NextPrayer, time_with_offset};
use crate::core::types::{Coordinates, UserData};
use crate::error::ErrorType;
use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Tz;
use ratatui::DefaultTerminal;
use rusqlite::Connection;
use std::path::PathBuf;
use std::time::Instant;

pub const MONTHS_THRESHOLD: usize = 2;

pub struct RunningData {
    pub date_time: DateTime<Tz>,
    pub method: String,
    pub hijri_date: NaiveHijriDate,
    pub prayer_times: DayPrayerTimes,
    pub calendar: CalendarPrayerTimes,
    pub city: String,
    pub country: String,
    pub utc_offset: i64,
    pub coordinates: Coordinates,
    pub next_prayer: NextPrayer,
    pub user_data: UserData,
    pub table_offset_date: NaiveDate,
    pub table_cursor_date: NaiveDate,
    pub visible_rows: usize,
    pub loading: bool,
}
impl RunningData {
    pub fn new(data: UserData) -> Result<Self, ErrorType> {
        let date_time = time_with_offset(
            &data.city.coordinates,
            data.city.timezone.utc_offset,
            Utc::now(),
        );
        let prayer_times = data.calculate(&date_time.date_naive())?;
        let calendar = data.calculate_batch(&date_time.date_naive())?;
        let method = data.country.method.clone().to_string();
        let hijri_date = NaiveHijriDate::from_gregorian_to_ummalqura(&date_time.date_naive())?;
        let city = data.city.name.clone();
        let country = data.country.name.clone();
        let utc_offset = data.city.timezone.utc_offset;
        let coordinates = data.city.coordinates.clone();
        let next_prayer = prayer_times.next_prayer(&date_time);

        let today = date_time.date_naive();

        let running = RunningData {
            date_time,
            method,
            prayer_times,
            calendar,
            city,
            hijri_date,
            utc_offset,
            country,
            coordinates,
            next_prayer,
            user_data: data,
            table_offset_date: today,
            table_cursor_date: today,
            visible_rows: 0,
            loading: false,
        };

        Ok(running)
    }

    pub fn total_days(&self) -> usize {
        self.calendar.months.iter().map(|m| m.days.len()).sum()
    }

    pub fn flat_index_of(&self, date: &NaiveDate) -> Option<usize> {
        let mut idx = 0;
        for month in self.calendar.months.iter().rev() {
            for day in &month.days {
                if day.date == *date {
                    return Some(idx);
                }
                idx += 1;
            }
        }
        None
    }

    pub fn get_day(&self, flat_index: usize) -> Option<&DayPrayerTimes> {
        let mut count = 0;
        for month in self.calendar.months.iter().rev() {
            if flat_index < count + month.days.len() {
                return month.days.get(flat_index - count);
            }
            count += month.days.len();
        }
        None
    }

    fn clamp_dates(&mut self) {
        let total = self.total_days();
        if total == 0 {
            return;
        }

        let first_date = self.get_day(0).map(|d| d.date);
        let last_date = self.get_day(total - 1).map(|d| d.date);

        if let Some(first) = first_date {
            if self.table_cursor_date < first {
                self.table_cursor_date = first;
            }
            if self.table_offset_date < first {
                self.table_offset_date = first;
            }
        }
        if let Some(last) = last_date {
            if self.table_cursor_date > last {
                self.table_cursor_date = last;
            }
            if self.table_offset_date > last {
                self.table_offset_date = last;
            }
        }
    }

    pub fn go_to_current(&mut self) {
        let now = time_with_offset(&self.coordinates, self.utc_offset, Utc::now());
        self.date_time = now;
        let today = self.date_time.date_naive();

        self.prayer_times = self
            .user_data
            .calculate(&today)
            .expect("failed to calculate today's prayer times");
        self.calendar = self
            .user_data
            .calculate_batch(&today)
            .expect("failed to recalculate calendar");

        self.table_cursor_date = today;
        self.table_offset_date = today;
        self.clamp_dates();
    }

    pub fn move_cursor(&mut self, delta: i32) -> Result<(), ErrorType> {
        let total = self.total_days();
        if total == 0 {
            return Ok(());
        }
        let cursor = self.flat_index_of(&self.table_cursor_date).unwrap_or(0);
        let offset = self.flat_index_of(&self.table_offset_date).unwrap_or(0);
        let vis = self.visible_rows.max(1);

        let new_cursor = if delta < 0 {
            cursor.saturating_sub(1)
        } else {
            (cursor + 1).min(total - 1)
        };

        let new_offset = if new_cursor < offset {
            new_cursor
        } else if new_cursor >= offset + vis {
            (new_cursor + 1).saturating_sub(vis)
        } else {
            offset
        };

        let max_offset = total.saturating_sub(vis);
        let new_offset = new_offset.min(max_offset);

        if let Some(day) = self.get_day(new_cursor) {
            self.table_cursor_date = day.date;
        }
        if new_offset != offset {
            if let Some(day) = self.get_day(new_offset) {
                self.table_offset_date = day.date;
            }
        }

        let _ = self.load_data_at_threshold(new_cursor);

        Ok(())
    }

    fn load_data_at_threshold(&mut self, cursor: usize) -> Result<(), ErrorType> {
        let total = self.total_days();
        if total == 0 {
            return Ok(());
        }

        if cursor <= MONTHS_THRESHOLD {
            self.loading = true;
            let _ = self.user_data.scroll_down(&mut self.calendar);
            self.loading = false;
            self.clamp_dates();
        }

        if total > 0 && total - cursor - 1 <= MONTHS_THRESHOLD {
            self.loading = true;
            let _ = self.user_data.scroll_up(&mut self.calendar);
            self.loading = false;
            self.clamp_dates();
        }

        Ok(())
    }

    pub fn go_to_top(&mut self) -> Result<(), ErrorType> {
        let total = self.total_days();
        if total == 0 {
            return Ok(());
        }
        let cursor = self.flat_index_of(&self.table_cursor_date).unwrap_or(0);
        let offset = self.flat_index_of(&self.table_offset_date).unwrap_or(0);
        let vis = self.visible_rows.max(1);

        if cursor > offset {
            if let Some(day) = self.get_day(offset) {
                self.table_cursor_date = day.date;
            }
        } else {
            let new_offset = offset.saturating_sub(vis);
            let d = self.get_day(new_offset).map(|day| day.date);
            if let Some(date) = d {
                self.table_offset_date = date;
                self.table_cursor_date = date;
            }
        }

        if let Some(idx) = self.flat_index_of(&self.table_cursor_date) {
            let _ = self.load_data_at_threshold(idx);
        }
        Ok(())
    }

    pub fn go_to_bottom(&mut self) -> Result<(), ErrorType> {
        let total = self.total_days();
        if total == 0 {
            return Ok(());
        }
        let cursor = self.flat_index_of(&self.table_cursor_date).unwrap_or(0);
        let offset = self.flat_index_of(&self.table_offset_date).unwrap_or(0);
        let vis = self.visible_rows.max(1);
        let bottom = (offset + vis)
            .saturating_sub(1)
            .min(total.saturating_sub(1));

        if cursor < bottom {
            if let Some(day) = self.get_day(bottom) {
                self.table_cursor_date = day.date;
            }
        } else {
            let new_offset = (offset + vis).min(total.saturating_sub(vis));
            let new_bottom = (new_offset + vis).saturating_sub(1).min(total - 1);
            let offset_date = self.get_day(new_offset).map(|d| d.date);
            let cursor_date = self.get_day(new_bottom).map(|d| d.date);
            if let Some(d) = offset_date {
                self.table_offset_date = d;
            }
            if let Some(d) = cursor_date {
                self.table_cursor_date = d;
            }
        }

        if let Some(idx) = self.flat_index_of(&self.table_cursor_date) {
            let _ = self.load_data_at_threshold(idx);
        }
        Ok(())
    }
}

pub struct SetupData {
    pub query: String,
    pub cursor: usize,
    pub results: Vec<(i64, String, String, String)>,
    pub selected: usize,
    pub last_input: Option<Instant>,
    pub search_triggered: bool,
    pub db: Connection,
    config_path: PathBuf,
}

impl SetupData {
    pub fn search(&mut self) -> Result<(), ErrorType> {
        if self.query.is_empty() {
            self.results.clear();
            self.selected = 0;
            return Ok(());
        }
        self.results = search_city_details(&self.db, &self.query)?;
        self.selected = self.selected.min(self.results.len().saturating_sub(1));
        Ok(())
    }

    pub fn select_current(&mut self) -> Result<Option<RunningData>, ErrorType> {
        if self.results.is_empty() {
            return Ok(None);
        }
        let (id, _, _, _) = &self.results[self.selected];
        let user_data = UserData::from_city_id(*id, &self.db)?;
        save_config(&user_data, &self.config_path)?;
        let running = RunningData::new(user_data)?;
        Ok(Some(running))
    }
}

impl Default for SetupData {
    fn default() -> Self {
        let config_path = get_config_path().expect("failed to get config path");
        Self {
            query: String::new(),
            cursor: 0,
            results: Vec::new(),
            selected: 0,
            last_input: None,
            search_triggered: false,
            db: db_connection().expect("failed to open db"),
            config_path,
        }
    }
}

pub enum AppState {
    Running(RunningData),
    Setup(SetupData),
    Error(ErrorType),
}

pub struct App {
    pub state: AppState,
    pub exit: bool,
    pub overlay: Option<SetupData>,
    pub pending_clear: bool,
}

impl App {
    pub fn new() -> Self {
        let file_path = match get_config_path() {
            Ok(path) => path,
            Err(e) => {
                return App {
                    state: AppState::Error(e),
                    exit: false,
                    overlay: None,
                    pending_clear: false,
                };
            }
        };

        let state = match load_config::<UserData>(&file_path) {
            Ok(data) => match RunningData::new(data) {
                Ok(running) => AppState::Running(running),
                Err(e) => AppState::Error(e),
            },
            Err(ErrorType::ConfigFileNotFound) => AppState::Setup(SetupData::default()),
            Err(e) => AppState::Error(e),
        };

        App {
            state,
            exit: false,
            overlay: None,
            pending_clear: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), ErrorType> {
        crossterm::execute!(terminal.backend_mut(), crossterm::event::EnableMouseCapture)?;
        let result = self.run_inner(terminal);
        crossterm::execute!(
            terminal.backend_mut(),
            crossterm::event::DisableMouseCapture
        )?;
        result
    }

    fn run_inner(&mut self, terminal: &mut DefaultTerminal) -> Result<(), ErrorType> {
        while !self.exit {
            if self.pending_clear {
                terminal.clear()?;
                self.pending_clear = false;
            }
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_event()?;
            self.handle_tick();
        }
        Ok(())
    }
}
