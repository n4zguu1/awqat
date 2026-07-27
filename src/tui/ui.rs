use crate::tui::app::{App, AppState, RunningData};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::Color;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Cell, Row, Table, TitlePosition, Widget};
use std::vec;

impl App {
    // the app should have max/min dimensions
    // when min dimensions is hit, show another screen
    // when max hit, the space get filled with another color or style.
    // the main content should stick to vertical center and horizontal center
    pub fn draw(&self, frame: &mut Frame) {
        match &self.state {
            AppState::Running(running_data) => running_data.draw(frame.area(), frame.buffer_mut()),
            AppState::Error(error) => {}
            AppState::Settings => {}
            AppState::Setup(setup_data) => {}
            AppState::Loading => {}
        }
    }

    fn handle_events() {}
}

impl RunningData {
    fn draw(&self, area: Rect, buffer: &mut Buffer) {
        let [main, table] =
            Layout::vertical(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
                .areas(area);
        let main_block = Block::new()
            .title("today")
            .title_position(TitlePosition::Top)
            .borders(Borders::ALL)
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);
        let inner_main = main_block.inner(main);

        let all_days_block = Block::new()
            .title("all days")
            .title_position(TitlePosition::Top)
            .borders(Borders::ALL)
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);
        let inner_all_days = all_days_block.inner(table);
        main_block.render(main, buffer);
        all_days_block.render(table, buffer);
        self.draw_main(inner_main, buffer);
        self.draw_table(inner_all_days, buffer);
    }
    fn draw_main(&self, main_area: Rect, buffer: &mut Buffer) {
        let time = self.prayer_times.as_string_with_timezone();
        let time_now = self.date_time.time().format("%I:%M:%S %P").to_string();
        let method = self.method.to_string();
        let date = self
            .date_time
            .date_naive()
            .format("%A %-d %B %C%y")
            .to_string();
        let date_hijri = format!(
            "{} {} {}",
            self.hijri_date.day, self.hijri_date.month_name, self.hijri_date.year
        );
        let [
        next_prayer_area,
        clock_area,
        day_prayers_area,
        meta_data_area,
        ] = Layout::vertical(vec![
            Constraint::Length(1),
            Constraint::Length(4),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
            .areas(main_area);

        let next_prayer_txt = Line::raw("Maghreb at 2 hours and 31 minutes")
            .alignment(Alignment::Left)
            .style(Color::Yellow);

        let clock_txt = Line::raw(time_now).alignment(Alignment::Left);

        let day_prayers_block = Block::new()
            .borders(Borders::BOTTOM | Borders::TOP)
            .border_type(BorderType::Plain);
        let day_prayers_inner = day_prayers_block.inner(day_prayers_area);
        // table defintion
        let header = Row::new(vec![
            Cell::from(Line::from("Fajr").centered()),
            Cell::from(Line::from("Sunrise").centered()),
            Cell::from(Line::from("Dhuhr").centered()),
            Cell::from(Line::from("Asr").centered()),
            Cell::from(Line::from("Maghrib").centered()),
            Cell::from(Line::from("Isha").centered()),
        ]);

        let row = [Row::new(vec![
            Cell::from(Line::from(time[0].clone()).centered()),
            Cell::from(Line::from(time[1].clone()).centered()),
            Cell::from(Line::from(time[2].clone()).centered()),
            Cell::from(Line::from(time[3].clone()).centered()),
            Cell::from(Line::from(time[4].clone()).centered()),
            Cell::from(Line::from(time[5].clone()).centered()),
        ])];
        let column_width = vec![Constraint::Ratio(1, 6); 6];
        let today_prayers = Table::new(row, column_width)
            .header(header)
            .style(Color::White);
        let [method_area, date_area, hijri_date_area] =
            Layout::vertical(vec![Constraint::Length(1); 3]).areas(meta_data_area);
        let method_txt = Line::from(method);
        let date_txt = Line::from(date);
        let hijri_date_txt = Line::from(date_hijri);


        method_txt.render(method_area, buffer);
        date_txt.render(date_area, buffer);
        hijri_date_txt.render(hijri_date_area, buffer);
        next_prayer_txt.render(next_prayer_area, buffer);
        clock_txt.render(clock_area, buffer);
        day_prayers_block.render(day_prayers_area, buffer);
        today_prayers.render(day_prayers_inner, buffer);
    }
    fn draw_table(&self, table_area: Rect, buffer: &mut Buffer) {}
}
