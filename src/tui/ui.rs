use crate::tui::app::{App, AppState, RunningData, SetupData};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Cell, Row, Table,
};
use tui_big_text::{BigText, PixelSize};

const PRAYER_NAMES: &[&str] = &["Fajr", "Sunrise", "Dhuhr", "Asr", "Maghrib", "Isha"];

impl App {
    pub fn draw(&self, frame: &mut Frame) {
        match &self.state {
            AppState::Running(data) => data.draw(frame),
            AppState::Setup(data) => data.draw(frame),
            AppState::Error(e) => Self::draw_error(frame, e),
        }
    }

    fn draw_error(frame: &mut Frame, e: &crate::error::ErrorType) {}
}

impl SetupData {
    fn draw(&self, frame: &mut Frame) {}
}

impl RunningData {
    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        let [header_area, hero_area, sep_area, table_area, footer_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(16),
            Constraint::Length(1),
            Constraint::Min(8),
            Constraint::Length(1),
        ])
            .areas(area);

        self.draw_header(frame, header_area);
        self.draw_hero(frame, hero_area);
        self.draw_separator(frame, sep_area);
        self.draw_monthly_table(frame, table_area);
        self.draw_footer(frame, footer_area);
    }

    fn draw_header(&self, frame: &mut Frame, area: Rect) {
        let line = Line::from(vec![
            Span::styled(
                " awqat",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" v", Style::default().fg(Color::DarkGray)),
            Span::styled(
                env!("CARGO_PKG_VERSION"),
                Style::default().fg(Color::DarkGray),
            ),
        ]);
        frame.render_widget(line, area);
    }

    fn draw_hero(&self, frame: &mut Frame, area: Rect) {
        let [clock_area, prayers_area, meta_area] = Layout::vertical([
            Constraint::Length(7),
            Constraint::Length(5),
            Constraint::Fill(1),
        ])
            .areas(area);

        self.draw_clock(frame, clock_area);
        self.draw_prayers(frame, prayers_area);
        self.draw_meta(frame, meta_area);
    }

    fn draw_clock(&self, frame: &mut Frame, area: Rect) {
        let time_now = self.date_time.time().format("%I:%M:%S %P").to_string();
        let big_clock = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .style(Style::new().white())
            .centered()
            .lines(vec![Line::from(time_now)])
            .build();
        frame.render_widget(big_clock, area);
    }

    fn draw_prayers(&self, frame: &mut Frame, area: Rect) {
        let times = self.prayer_times.as_string_with_timezone();
        let next_idx = self.next_prayer_index();

        let header = Row::new(PRAYER_NAMES.iter().enumerate().map(|(i, n)| {
            let style = if Some(i) == next_idx {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            Cell::from(Line::from(*n).centered()).style(style)
        }));

        let row = Row::new(times.iter().enumerate().map(|(i, t)| {
            let style = if Some(i) == next_idx {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            Cell::from(Line::from(t.as_str()).centered()).style(style)
        }));

        let widths = vec![Constraint::Ratio(1, 6); 6];
        let table = Table::new(vec![row], widths).header(header);
        frame.render_widget(table, area);
    }

    fn draw_meta(&self, frame: &mut Frame, area: Rect) {
        let date_str = self
            .date_time
            .date_naive()
            .format("%A, %-d %B %Y")
            .to_string();
        let hijri_str = format!(
            "{} {} {}",
            self.hijri_date.day, self.hijri_date.month_name, self.hijri_date.year
        );

        let [left_area, right_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(area);

        let left = Line::from(vec![
            Span::styled(
                &self.city,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(&self.method, Style::default().fg(Color::DarkGray)),
        ]);
        let right = Line::from(vec![
            Span::styled(&date_str, Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled(&hijri_str, Style::default().fg(Color::Yellow)),
        ])
            .alignment(Alignment::Right);

        frame.render_widget(left, left_area);
        frame.render_widget(right, right_area);
    }

    fn draw_separator(&self, frame: &mut Frame, area: Rect) {
        let line = Line::from(Span::styled(
            "─".repeat(area.width as usize),
            Style::default().fg(Color::DarkGray),
        ));
        frame.render_widget(line, area);
    }

    fn draw_monthly_table(&self, frame: &mut Frame, area: Rect) {}

    fn draw_footer(&self, frame: &mut Frame, area: Rect) {
        let help = Line::from(Span::styled(
            " q/Ctrl+C: quit  j/k/\u{2195}/\u{2191}: scroll  PgUp/PgDn: page  g/G: top/bottom",
            Style::default().fg(Color::DarkGray),
        ));
        frame.render_widget(help, area);
    }

    fn next_prayer_index(&self) -> Option<usize> {
        let now = self.date_time.time();
        let times = [
            self.prayer_times.fajr.time(),
            self.prayer_times.sunrise.time(),
            self.prayer_times.dhuhr.time(),
            self.prayer_times.asr.time(),
            self.prayer_times.maghrib.time(),
            self.prayer_times.isha.time(),
        ];
        times.iter().position(|t| *t > now)
    }
}
