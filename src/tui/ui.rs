use crate::tui::app::{App, AppState, RunningData, SetupData};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Row, Table};
use tui_big_text::{BigText, PixelSize};

const PRAYER_NAMES: &[&str] = &["Fajr", "Sunrise", "Dhuhr", "Asr", "Maghrib", "Isha"];

fn format_gmt(offset_seconds: i64) -> String {
    let sign = if offset_seconds >= 0 { '+' } else { '-' };
    let abs = offset_seconds.abs();
    let hours = abs / 3600;
    let minutes = (abs % 3600) / 60;
    if minutes > 0 {
        format!("GMT{}{:02}:{:02}", sign, hours, minutes)
    } else {
        format!("GMT{}{:02}", sign, hours)
    }
}

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

        // Optimized Hero area constraints to give BigText and the Prayer Table proper vertical space
        let [header_area, hero_area, sep_area, table_area, footer_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(11),
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
            .spacing(1)
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
                " 🕌 awqat",
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
            Constraint::Length(4),
            Constraint::Length(5),
            Constraint::Length(2),
        ])
            .areas(area);

        self.draw_clock(frame, clock_area);
        self.draw_prayers(frame, prayers_area);
        self.draw_meta(frame, meta_area);
    }

    fn draw_clock(&self, frame: &mut Frame, area: Rect) {
        let time_now = self.date_time.time().format("%I:%M %P").to_string();
        let big_clock = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .style(Style::new().cyan().bold())
            .centered()
            .lines(vec![Line::from(time_now)])
            .build();
        frame.render_widget(big_clock, area);
    }

    fn draw_prayers(&self, frame: &mut Frame, area: Rect) {
        let active_prayer = self.next_prayer.prayer.to_string();

        let prayers = [
            ("Fajr", &self.prayer_times.fajr),
            ("Sunrise", &self.prayer_times.sunrise),
            ("Dhuhr", &self.prayer_times.dhuhr),
            ("Asr", &self.prayer_times.asr),
            ("Maghrib", &self.prayer_times.maghrib),
            ("Isha", &self.prayer_times.isha),
        ];

        // 1. Build Header Cells (Centered)
        let header_cells = prayers.iter().map(|(name, _)| {
            Cell::from(Line::from(*name).alignment(Alignment::Center)).style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
        });

        let header = Row::new(header_cells).height(1).bottom_margin(1);

        // 2. Build Value Cells with active prayer highlight
        let value_cells = prayers.iter().map(|(name, time)| {
            let formatted_time = time.format("%I:%M %p").to_string();
            let is_active = *name == active_prayer;

            let style = if is_active {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };

            Cell::from(Line::from(formatted_time).alignment(Alignment::Center)).style(style)
        });

        let body = vec![Row::new(value_cells).height(1)];
        let widths = [Constraint::Ratio(1, 6); 6];

        let block = Block::default()
            .title(" 🕌 Prayer Times ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray));

        let table = Table::new(body, widths)
            .header(header)
            .column_spacing(1)
            .block(block);

        let table_width = area.width.min(80);
        let table_height = 5;

        let centered = Rect {
            x: area.x + (area.width.saturating_sub(table_width)) / 2,
            y: area.y + (area.height.saturating_sub(table_height)) / 2,
            width: table_width,
            height: table_height,
        };

        frame.render_widget(table, centered);
    }

    fn draw_meta(&self, frame: &mut Frame, area: Rect) {
        let [city_area, method_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(area);

        let country = &self.country;
        let city_line = Line::from(vec![
            Span::styled("📍 ", Style::default()),
            Span::styled(
                self.city.as_str(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(", ", Style::default().fg(Color::DarkGray)),
            Span::styled(country.as_str(), Style::default().fg(Color::Cyan)),
        ])
            .alignment(Alignment::Center);
        frame.render_widget(city_line, city_area);

        let gmt = format_gmt(self.utc_offset);
        let method_line = Line::from(vec![
            Span::styled("Based on: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                self.method.as_str(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled(&gmt, Style::default().fg(Color::Yellow)),
        ])
            .alignment(Alignment::Center);
        frame.render_widget(method_line, method_area);
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
        let help = Line::from(vec![
            Span::styled(" q/Ctrl+C", Style::default().fg(Color::Cyan)),
            Span::styled(": quit  ", Style::default().fg(Color::DarkGray)),
            Span::styled("j/k/↑/↓", Style::default().fg(Color::Cyan)),
            Span::styled(": scroll  ", Style::default().fg(Color::DarkGray)),
            Span::styled("c", Style::default().fg(Color::Cyan)),
            Span::styled(": today  ", Style::default().fg(Color::DarkGray)),
            Span::styled("g/G", Style::default().fg(Color::Cyan)),
            Span::styled(": top/bottom", Style::default().fg(Color::DarkGray)),
        ])
            .alignment(Alignment::Center);

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