use crate::tui::app::SetupData;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};

use super::theme;

impl SetupData {
    fn draw_widget(&self, frame: &mut Frame, area: Rect, title: &str, show_bg: bool) {
        if show_bg {
            frame.render_widget(Clear, area);
            let bg_block = Block::default().style(Style::default().bg(theme::CARD_BG));
            frame.render_widget(bg_block, area);
        }

        let [input_area, results_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)])
                .spacing(1)
                .areas(area);

        // Search Input Box
        let input_block = Block::default()
            .title(format!(" {title} "))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::CYAN));

        let input_text = if self.query.is_empty() {
            Line::from(vec![
                Span::styled(" 🔍 ", Style::default().fg(theme::CYAN)),
                Span::styled(
                    "Type a city name...",
                    Style::default().fg(theme::TEXT_MUTED),
                ),
            ])
        } else {
            Line::from(vec![
                Span::styled(" 🔍 ", Style::default().fg(theme::CYAN)),
                Span::styled(
                    &self.query,
                    Style::default()
                        .fg(theme::TEXT_PRIMARY)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
        };

        let input_para = Paragraph::new(input_text).block(input_block);
        frame.render_widget(input_para, input_area);

        // Results Box
        let count_label = format!(" {} found ", self.results.len());
        let results_block = Block::default()
            .title(format!(" Results ({}) ", count_label.trim()))
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::BORDER_INACTIVE));

        if self.results.is_empty() {
            let msg = if self.query.is_empty() {
                Line::from(Span::styled(
                    "Start typing to search for a city...",
                    Style::default().fg(theme::TEXT_MUTED),
                ))
            } else {
                Line::from(Span::styled(
                    "No matching cities found",
                    Style::default().fg(theme::RED),
                ))
            };
            let para = Paragraph::new(msg)
                .alignment(Alignment::Center)
                .block(results_block);
            frame.render_widget(para, results_area);
        } else {
            let mut items = Vec::with_capacity(self.results.len());
            for (i, (_, city, region, country)) in self.results.iter().enumerate() {
                let is_selected = i == self.selected;
                let prefix = if is_selected { " ▶ " } else { "   " };
                let display = format!("{}{} —, {} —, {}", prefix, city, region, country);

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(theme::CYAN)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme::TEXT_PRIMARY)
                };

                items.push(Line::from(Span::styled(display, style)));
            }
            let list = Paragraph::new(items).block(results_block);
            frame.render_widget(list, results_area);
        }
    }

    pub(super) fn draw_overlay(&self, frame: &mut Frame) {
        let area = frame.area();
        let w = 64u16.min(area.width.saturating_sub(4));
        let h = 18u16.min(area.height.saturating_sub(4));
        let x = area.x + (area.width.saturating_sub(w)) / 2;
        let y = area.y + (area.height.saturating_sub(h)) / 2;
        let rect = Rect {
            x,
            y,
            width: w,
            height: h,
        };

        // Dim background
        let dim_block = Block::default().style(Style::default().bg(theme::BG_OVERLAY));
        frame.render_widget(dim_block, area);

        self.draw_widget(frame, rect, "Change City Location", true);
    }

    pub(super) fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        let w = 64u16.min(area.width.saturating_sub(4));
        let h = 18u16.min(area.height.saturating_sub(4));
        let x = area.x + (area.width.saturating_sub(w)) / 2;
        let y = area.y + (area.height.saturating_sub(h)) / 2;
        let rect = Rect {
            x,
            y,
            width: w,
            height: h,
        };

        self.draw_widget(frame, rect, "Welcome to AWQAT — Select City", false);
    }
}
