use crate::tui::app::RunningData;
use chrono::Datelike;
use chrono_tz::Tz;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table};
use tui_big_text::{BigText, PixelSize};

use super::{format_gmt, theme};

impl RunningData {
    pub(super) fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();

        // High-level vertical dashboard layout
        let [header_area, hero_area, table_area, footer_area] = Layout::vertical([
            Constraint::Length(1),  // Top status bar
            Constraint::Length(11), // Hero Dashboard (Clock + Prayer Cards Split)
            Constraint::Fill(1),    // Interactive Monthly Table
            Constraint::Length(1),  // Keybindings Footer Bar
        ])
            .spacing(1)
            .areas(area);

        self.draw_header(frame, header_area);
        self.draw_hero(frame, hero_area);
        self.draw_monthly_table(frame, table_area);
        self.draw_footer(frame, footer_area);
    }

    // ------------------------------------------------------------------------
    // TOP HEADER BAR
    // ------------------------------------------------------------------------
    fn draw_header(&self, frame: &mut Frame, area: Rect) {
        let [left_area, right_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(area);

        // App Branding
        let brand = Line::from(vec![
            Span::styled(
                " 🕌 AWQAT ",
                Style::default()
                    .fg(Color::Black)
                    .bg(theme::CYAN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" v{} ", env!("CARGO_PKG_VERSION")),
                Style::default().fg(theme::TEXT_MUTED).bg(theme::CARD_BG),
            ),
        ]);

        // Location & Config Summary
        let meta = Line::from(vec![
            Span::styled("📍 ", Style::default()),
            Span::styled(
                format!("{}, {}", self.city, self.country),
                Style::default()
                    .fg(theme::TEXT_PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" • ", Style::default().fg(theme::TEXT_MUTED)),
            Span::styled(&self.method, Style::default().fg(theme::CYAN)),
            Span::styled(
                format!(" ({}) ", format_gmt(self.utc_offset)),
                Style::default().fg(theme::TEXT_MUTED),
            ),
        ])
            .alignment(Alignment::Right);

        frame.render_widget(Paragraph::new(brand), left_area);
        frame.render_widget(Paragraph::new(meta), right_area);
    }

    // ------------------------------------------------------------------------
    // HERO DASHBOARD PANEL (SPLIT HORIZONTALLY)
    // ------------------------------------------------------------------------
    fn draw_hero(&self, frame: &mut Frame, area: Rect) {
        let [clock_card_area, prayers_card_area] =
            Layout::horizontal([Constraint::Percentage(42), Constraint::Percentage(58)])
                .spacing(1)
                .areas(area);

        self.draw_clock_card(frame, clock_card_area);
        self.draw_prayers_card(frame, prayers_card_area);
    }

    // Left Hero: Dates, Big Clock, and Countdown Badge
    fn draw_clock_card(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::BORDER_ACTIVE));

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let [dates_layout, clock_layout, remaining_layout] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(4),
            Constraint::Length(2),
        ])
            .spacing(1)
            .areas(inner_area);

        // Gregorian & Hijri Header
        let greg = self.date_time.format("%a, %d %b %Y").to_string();
        let hijri = format!("{}", self.hijri_date);
        let date_line = Line::from(vec![
            Span::styled(
                greg,
                Style::default()
                    .fg(theme::TEXT_PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("  │  ", Style::default().fg(theme::TEXT_MUTED)),
            Span::styled("🌙 ", Style::default()),
            Span::styled(
                hijri,
                Style::default()
                    .fg(theme::GOLD)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
            .alignment(Alignment::Center);

        frame.render_widget(Paragraph::new(date_line), dates_layout);

        // Big Digital Clock
        let time_now = self.date_time.time().format("%I:%M %P").to_string();
        let big_clock = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .style(Style::new().fg(theme::CYAN).bold())
            .centered()
            .lines(vec![Line::from(time_now)])
            .build();

        frame.render_widget(big_clock, clock_layout);

        // Next Prayer Countdown Pill
        let remaining = self.next_prayer.remaining.abs();
        let hours = remaining.num_hours();
        let minutes = remaining.num_minutes() % 60;
        let remaining_str = if hours > 0 {
            format!("{}h {:02}m", hours, minutes)
        } else if minutes > 0 {
            format!("{}m", minutes)
        } else {
            format!("{}s", remaining.num_seconds())
        };

        let countdown_line = Line::from(vec![
            Span::styled(
                " ⏳ NEXT: ",
                Style::default()
                    .fg(theme::TEXT_MUTED)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" {} ", self.next_prayer.prayer),
                Style::default()
                    .fg(Color::Black)
                    .bg(theme::GOLD)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" in {} ", remaining_str),
                Style::default()
                    .fg(theme::TEXT_PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
            .alignment(Alignment::Center);

        frame.render_widget(Paragraph::new(countdown_line), remaining_layout);
    }

    // Right Hero: Today's Prayer Times Grid
    fn draw_prayers_card(&self, frame: &mut Frame, area: Rect) {
        let title_line = Line::from(Span::styled(
            " ⏱ Today's Schedule ",
            Style::default()
                .fg(theme::GOLD)
                .add_modifier(Modifier::BOLD),
        ));
        let block = Block::default()
            .title(title_line)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::BORDER_ACTIVE));

        let active = self.next_prayer.prayer.to_string();
        let prayers = [
            ("Fajr", &self.prayer_times.fajr),
            ("Sunrise", &self.prayer_times.sunrise),
            ("Dhuhr", &self.prayer_times.dhuhr),
            ("Asr", &self.prayer_times.asr),
            ("Maghrib", &self.prayer_times.maghrib),
            ("Isha", &self.prayer_times.isha),
        ];

        let header_cells = prayers.iter().map(|(name, _)| {
            let is_active = *name == active;
            Cell::from(Line::from(*name).alignment(Alignment::Center)).style(
                Style::default()
                    .fg(if is_active {
                        theme::GOLD
                    } else {
                        theme::TEXT_MUTED
                    })
                    .add_modifier(Modifier::BOLD),
            )
        });
        let header = Row::new(header_cells).height(1).bottom_margin(1);

        let value_cells = prayers.iter().map(|(name, time)| {
            let formatted_time = time.format("%I:%M %p").to_string();
            let is_active = *name == active;

            let style = if is_active {
                Style::default()
                    .fg(Color::Black)
                    .bg(theme::GOLD)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(theme::TEXT_PRIMARY)
                    .add_modifier(Modifier::BOLD)
            };

            Cell::from(Line::from(formatted_time).alignment(Alignment::Center)).style(style)
        });

        let body = vec![Row::new(value_cells).height(1)];
        let widths = [Constraint::Ratio(1, 6); 6];

        let table = Table::new(body, widths).header(header).column_spacing(1);

        let inner = block.inner(area);
        let content_height = 3u16;
        let top_pad = inner.height.saturating_sub(content_height) / 2;
        let [_, content_area, _] = Layout::vertical([
            Constraint::Length(top_pad),
            Constraint::Length(content_height),
            Constraint::Fill(1),
        ])
            .areas(inner);

        frame.render_widget(&block, area);
        frame.render_widget(table, content_area);
    }

    // ------------------------------------------------------------------------
    // MONTHLY SCHEDULE TABLE
    // ------------------------------------------------------------------------
    fn draw_monthly_table(&mut self, frame: &mut Frame, area: Rect) {
        let inner_h = area.height.saturating_sub(2);
        let rows_available = inner_h.saturating_sub(2) as usize;
        if rows_available == 0 {
            return;
        }

        let total = self.total_days();
        if total == 0 {
            return;
        }

        let max_offset = total.saturating_sub(rows_available);
        let cursor_idx = self.flat_index_of(&self.table_cursor_date).unwrap_or(0);
        let offset_idx = self.flat_index_of(&self.table_offset_date).unwrap_or(0);

        let offset = if self.visible_rows == 0 {
            let mid = rows_available / 2;
            cursor_idx.saturating_sub(mid).min(max_offset)
        } else {
            offset_idx.min(max_offset)
        };

        if let Some(day) = self.get_day(offset) {
            self.table_offset_date = day.date;
        }
        self.visible_rows = rows_available;

        let end = (offset + rows_available).min(total);
        let visible = end - offset;

        if visible == 0 {
            return;
        }

        let cursor_idx = if cursor_idx < offset {
            if let Some(day) = self.get_day(offset) {
                self.table_cursor_date = day.date;
            }
            offset
        } else if cursor_idx >= end {
            if let Some(day) = self.get_day(end - 1) {
                self.table_cursor_date = day.date;
            }
            end - 1
        } else {
            cursor_idx
        };

        let cursor_rel = cursor_idx - offset;

        // Header configuration
        let headers = [
            "Gregorian",
            "Hijri",
            "Fajr",
            "Sunrise",
            "Dhuhr",
            "Asr",
            "Maghrib",
            "Isha",
        ];
        let header_cells = headers.iter().map(|h| {
            Cell::from(Line::from(*h).alignment(Alignment::Center)).style(
                Style::default()
                    .fg(theme::CYAN)
                    .add_modifier(Modifier::BOLD),
            )
        });
        let header = Row::new(header_cells).height(1).bottom_margin(1);

        // Data rows build
        let mut rows = Vec::with_capacity(visible);
        for i in 0..visible {
            let flat = offset + i;
            if let Some(day) = self.get_day(flat) {
                let is_highlighted = i == cursor_rel;
                let is_even = i % 2 == 0;

                let greg = format!(
                    "{} {} {}",
                    day.date.day(),
                    day.date.format("%b"),
                    day.date.year()
                );
                let hijri = format!("{}", day.hijri);
                let fmt_time = |dt: &chrono::DateTime<Tz>| dt.format("%I:%M %p").to_string();

                let row_style = if is_highlighted {
                    Style::default()
                        .fg(Color::Black)
                        .bg(theme::CYAN)
                        .add_modifier(Modifier::BOLD)
                } else if is_even {
                    Style::default().fg(theme::TEXT_PRIMARY)
                } else {
                    Style::default().fg(theme::TEXT_MUTED)
                };

                let prefix = if is_highlighted { "▶ " } else { "" };
                let cells = vec![
                    Cell::from(
                        Line::from(format!("{}{}", prefix, greg)).alignment(Alignment::Center),
                    ),
                    Cell::from(Line::from(hijri).alignment(Alignment::Center)),
                    Cell::from(Line::from(fmt_time(&day.fajr)).alignment(Alignment::Center)),
                    Cell::from(Line::from(fmt_time(&day.sunrise)).alignment(Alignment::Center)),
                    Cell::from(Line::from(fmt_time(&day.dhuhr)).alignment(Alignment::Center)),
                    Cell::from(Line::from(fmt_time(&day.asr)).alignment(Alignment::Center)),
                    Cell::from(Line::from(fmt_time(&day.maghrib)).alignment(Alignment::Center)),
                    Cell::from(Line::from(fmt_time(&day.isha)).alignment(Alignment::Center)),
                ];

                rows.push(Row::new(cells).height(1).style(row_style));
            }
        }

        let widths = [
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(11),
            Constraint::Percentage(11),
            Constraint::Percentage(11),
            Constraint::Percentage(11),
            Constraint::Percentage(11),
            Constraint::Percentage(11),
        ];

        let table = Table::new(rows, widths).header(header).column_spacing(1);

        let scroll_title = format!(" 📅 Monthly Schedule [{}/{}] ", cursor_idx + 1, total);
        let title_line = Line::from(Span::styled(
            scroll_title,
            Style::default()
                .fg(theme::GOLD)
                .add_modifier(Modifier::BOLD),
        ));
        let block = Block::default()
            .title(title_line)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::BORDER_ACTIVE));

        if self.loading {
            let loading =
                Paragraph::new(Line::from(" Loading schedule... ").alignment(Alignment::Center))
                    .block(block)
                    .style(
                        Style::default()
                            .fg(theme::GOLD)
                            .add_modifier(Modifier::BOLD),
                    );
            frame.render_widget(loading, area);
        } else {
            frame.render_widget(table.block(block), area);
        }
    }

    // ------------------------------------------------------------------------
    // FOOTER KEYBINDINGS BAR
    // ------------------------------------------------------------------------
    fn draw_footer(&self, frame: &mut Frame, area: Rect) {
        let keys = [
            (" Q ", "Quit"),
            (" J/K ", "Scroll"),
            (" C ", "Today"),
            (" S ", "Search City"),
            (" G/g ", "Top/Bottom"),
        ];

        let mut spans = Vec::new();
        for (i, (key, label)) in keys.iter().enumerate() {
            spans.push(Span::styled(
                *key,
                Style::default()
                    .fg(Color::Black)
                    .bg(theme::TEXT_PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(
                format!(" {} ", label),
                Style::default().fg(theme::TEXT_MUTED),
            ));
            if i < keys.len() - 1 {
                spans.push(Span::styled(
                    " │ ",
                    Style::default().fg(theme::BORDER_INACTIVE),
                ));
            }
        }

        let help = Line::from(spans).alignment(Alignment::Center);
        frame.render_widget(Paragraph::new(help), area);
    }

    #[expect(dead_code)]
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
