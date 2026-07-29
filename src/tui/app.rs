use crate::core::date::NaiveHijriDate;
use crate::core::storage::{db_connection, get_config_path, load_config};
use crate::core::time::{CalendarPrayerTimes, DayPrayerTimes, NextPrayer, time_with_offset};
use crate::core::types::{Coordinates, UserData};
use crate::error::ErrorType;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use ratatui::DefaultTerminal;
use rusqlite::Connection;
use std::time::Instant;

const MONTHS_THRESHOLD: i32 = 2;

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
        let country = data.country.name;
        let utc_offset = data.city.timezone.utc_offset;
        let coordinates = data.city.coordinates;
        let next_prayer = prayer_times.next_prayer(&date_time);

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
        };

        Ok(running)
    }
}

pub struct SetupData {
    pub query: String,
    pub cursor: usize,
    pub results: Vec<(i64, String)>,
    pub selected: usize,
    pub last_input: Option<Instant>,
    pub search_triggered: bool,
    pub db: Connection,
}

impl Default for SetupData {
    fn default() -> Self {
        Self {
            query: String::new(),
            cursor: 0,
            results: Vec::new(),
            selected: 0,
            last_input: None,
            search_triggered: false,
            db: db_connection().expect("failed to open db"),
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
}

impl App {
    pub fn new() -> Self {
        let file_path = match get_config_path() {
            Ok(path) => path,
            Err(e) => {
                return App {
                    state: AppState::Error(e),
                    exit: false,
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

        App { state, exit: false }
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
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_event()?;
            self.handle_tick();
        }
        Ok(())
    }
}
