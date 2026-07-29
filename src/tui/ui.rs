use crate::tui::app::{App, AppState, RunningData, SetupData};
use chrono::Datelike;
use chrono_tz::Tz;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Clear, Paragraph, Row, Table};
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
    pub fn draw(&mut self, frame: &mut Frame) {
        match &mut self.state {
            AppState::Running(data) => {
                data.draw(frame);
                if let Some(overlay) = &self.overlay {
                    overlay.draw_overlay(frame);
                }
            }
            AppState::Setup(data) => data.draw(frame),
            AppState::Error(e) => Self::draw_error(frame, e),
        }
    }

    fn draw_error(_frame: &mut Frame, _e: &crate::error::ErrorType) {}
}

impl SetupData {
    fn draw_widget(&self, frame: &mut Frame, area: Rect, title: &str, show_bg: bool) {
        if show_bg {
            frame.render_widget(Clear, area);
            let bg_block = Block::default().style(Style::default().bg(Color::Black));
            frame.render_widget(bg_block, area);
        }

        let [input_area, results_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)])
                .spacing(1)
                .areas(area);

        let input_block = Block::default()
            .title(format!(" {title} "))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan));

        let input_text = if self.query.is_empty() {
            Line::from(Span::styled(
                " Type a city name...",
                Style::default().fg(Color::DarkGray),
            ))
        } else {
            Line::from(Span::styled(&self.query, Style::default().fg(Color::White)))
        };

        let input_para = Paragraph::new(input_text)
            .block(input_block)
            .style(Style::default());

        frame.render_widget(input_para, input_area);

        let count_label = if show_bg {
            format!(" {}", self.results.len())
        } else {
            format!(" {} found ", self.results.len())
        };
        let results_block = Block::default()
            .title(format!(" Results{}", count_label))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray));

        if self.results.is_empty() {
            let msg = if self.query.is_empty() {
                Line::from(Span::styled(
                    " Start typing to search for a city",
                    Style::default().fg(Color::DarkGray),
                ))
                    .alignment(Alignment::Center)
            } else {
                Line::from(Span::styled(
                    " No cities found",
                    Style::default().fg(Color::DarkGray),
                ))
                    .alignment(Alignment::Center)
            };
            let para = Paragraph::new(msg).block(results_block);
            frame.render_widget(para, results_area);
        } else {
            let mut items = Vec::with_capacity(self.results.len());
            for (i, (_, city, region, country)) in self.results.iter().enumerate() {
                let is_selected = i == self.selected;
                let display = format!("{} — {} — {}", city, region, country);
                let style = if is_selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Gray)
                };
                let line = Line::from(Span::styled(display, style));
                items.push(line);
            }
            let list = Paragraph::new(items)
                .block(results_block)
                .style(Style::default());
            frame.render_widget(list, results_area);
        }
    }

    fn draw_overlay(&self, frame: &mut Frame) {
        let area = frame.area();
        let w = 60u16.min(area.width.saturating_sub(4));
        let h = 18u16.min(area.height.saturating_sub(4));
        let x = area.x + (area.width - w) / 2;
        let y = area.y + (area.height - h) / 2;
        let rect = Rect { x, y, width: w, height: h };
        self.draw_widget(frame, rect, "🔍 Change City", true);
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        let w = 60u16.min(area.width.saturating_sub(4));
        let h = 18u16.min(area.height.saturating_sub(4));
        let x = area.x + (area.width - w) / 2;
        let y = area.y + (area.height - h) / 2;
        let rect = Rect { x, y, width: w, height: h };
        self.draw_widget(frame, rect, "🌍 Search City", false);
    }
}

impl RunningData {
    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();

        // Optimized vertical layout: beautifully spaced without needing explicit separator lines
        let [header_area, hero_area, table_area, footer_area] = Layout::vertical([
            Constraint::Length(1),  // Top branding bar
            Constraint::Length(13), // Hero (Meta + Dates + Clock + Prayers)
            Constraint::Fill(1),    // Expanding monthly table
            Constraint::Length(1),  // Footer shortcuts
        ])
            .spacing(1) // Automatically adds a clean blank line between sections
            .areas(area);

        self.draw_header(frame, header_area);
        self.draw_hero(frame, hero_area);
        self.draw_monthly_table(frame, table_area);
        self.draw_footer(frame, footer_area);
    }

    fn draw_header(&self, frame: &mut Frame, area: Rect) {
        let header_text = Line::from(vec![
            Span::styled(
                " 🕌 AWQAT ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" v{} ", env!("CARGO_PKG_VERSION")),
                Style::default().fg(Color::Gray).bg(Color::DarkGray),
            ),
        ]);

        frame.render_widget(Paragraph::new(header_text), area);
    }

    fn draw_hero(&self, frame: &mut Frame, area: Rect) {
        let [meta_area, dates_area, clock_area, prayers_area] = Layout::vertical([
            Constraint::Length(1), // Location and settings info
            Constraint::Length(1), // Gregorian + Hijri + Next prayer
            Constraint::Length(4), // Big Text Clock
            Constraint::Length(4), // Prayer horizontal minimal table
        ])
            .spacing(1)
            .areas(area);

        self.draw_meta(frame, meta_area);
        self.draw_dates_and_remaining(frame, dates_area);
        self.draw_clock(frame, clock_area);
        self.draw_prayers(frame, prayers_area);
    }

    fn draw_meta(&self, frame: &mut Frame, area: Rect) {
        let meta_line = Line::from(vec![
            Span::styled("📍 ", Style::default()),
            Span::styled(
                self.city.as_str(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(", ", Style::default().fg(Color::DarkGray)),
            Span::styled(self.country.as_str(), Style::default().fg(Color::Gray)),
            Span::styled("  •  ", Style::default().fg(Color::DarkGray)),
            Span::styled(self.method.as_str(), Style::default().fg(Color::Cyan)),
            Span::styled("  •  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format_gmt(self.utc_offset),
                Style::default().fg(Color::Gray),
            ),
        ])
            .alignment(Alignment::Center);

        frame.render_widget(Paragraph::new(meta_line), area);
    }

    fn draw_dates_and_remaining(&self, frame: &mut Frame, area: Rect) {
        let greg = self.date_time.format("%d %B %Y").to_string();
        let hijri = format!("{}", self.hijri_date);
        let prayer = format!("{}", self.next_prayer.prayer);
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

        let line = Line::from(vec![
            Span::styled("📅 ", Style::default()),
            Span::styled(&greg, Style::default().fg(Color::White)),
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled("🌙 ", Style::default()),
            Span::styled(&hijri, Style::default().fg(Color::White)),
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled("⏳ ", Style::default()),
            Span::styled("Next: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                prayer,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" in {remaining_str}"),
                Style::default().fg(Color::Gray),
            ),
        ])
            .alignment(Alignment::Center);

        frame.render_widget(Paragraph::new(line), area);
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

        // Headers styling
        let header_cells = prayers.iter().map(|(name, _)| {
            Cell::from(Line::from(*name).alignment(Alignment::Center)).style(
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
        });
        let header = Row::new(header_cells).height(1);

        // Value Cells highlighting next prayer prominently
        let value_cells = prayers.iter().map(|(name, time)| {
            let formatted_time = time.format("%I:%M %p").to_string();
            let is_active = *name == active_prayer;

            let style = if is_active {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            };

            Cell::from(Line::from(formatted_time).alignment(Alignment::Center)).style(style)
        });

        let body = vec![Row::new(value_cells).height(1)];
        let widths = [Constraint::Ratio(1, 6); 6];

        // Minimalist borders – removes the heavy box for a sleek divider feel
        let block = Block::default()
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Color::DarkGray));

        let table = Table::new(body, widths).header(header).column_spacing(1);

        let table_width = area.width.min(90);
        let table_height = 4; // Top border + Header + Row + Bottom border = 4 perfectly fits

        let centered = Rect {
            x: area.x + (area.width.saturating_sub(table_width)) / 2,
            y: area.y + (area.height.saturating_sub(table_height)) / 2,
            width: table_width,
            height: table_height,
        };

        frame.render_widget(table.block(block), centered);
    }

    fn draw_monthly_table(&mut self, frame: &mut Frame, area: Rect) {
        let inner_h = area.height.saturating_sub(2);
        // Adjusted for the new bottom_margin(1) in the header
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

        // Build elegant table header
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
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
        });
        let header = Row::new(header_cells).height(1).bottom_margin(1); // Breathing room

        // Build data rows with zebra striping
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
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else if is_even {
                    Style::default().fg(Color::White)
                } else {
                    Style::default().fg(Color::Gray) // Zebra striping for better readability
                };

                let cells = vec![
                    Cell::from(Line::from(greg).alignment(Alignment::Center)),
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

        // Precise percentages instead of generic fractions
        let widths = [
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(12),
            Constraint::Percentage(12),
            Constraint::Percentage(12),
            Constraint::Percentage(12),
            Constraint::Percentage(12),
            Constraint::Percentage(12),
        ];

        let table = Table::new(rows, widths).header(header).column_spacing(2); // increased spacing

        let block = Block::default()
            .title(" 📅 Monthly Schedule ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan));

        if self.loading {
            let loading = Paragraph::new(Line::from(" Loading... ").alignment(Alignment::Center))
                .block(block) // reuse the same beautiful frame
                .style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                );
            frame.render_widget(loading, area);
        } else {
            frame.render_widget(table.block(block), area);
        }
    }

    fn draw_footer(&self, frame: &mut Frame, area: Rect) {
        // Styled to look like sleek keyboard shortcuts
        let help = Line::from(vec![
            Span::styled(
                " q/Ctrl+q/Ctrl+c ",
                Style::default()
                    .bg(Color::White)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Quit   ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                " j/k/↑/↓ ",
                Style::default()
                    .bg(Color::White)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Scroll   ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                " c ",
                Style::default()
                    .bg(Color::White)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Today   ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                " s ",
                Style::default()
                    .bg(Color::White)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Search   ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                " G/g ",
                Style::default()
                    .bg(Color::White)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Top/Bottom ", Style::default().fg(Color::DarkGray)),
        ])
            .alignment(Alignment::Center);

        frame.render_widget(Paragraph::new(help), area);
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
