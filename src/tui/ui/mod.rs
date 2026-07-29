use crate::tui::app::{App, AppState};
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};

const MIN_WIDTH: u16 = 98;
const MIN_HEIGHT: u16 = 31;

pub mod running;
pub mod setup;

mod theme {
    use ratatui::style::Color;

    #[expect(dead_code)]
    pub const BG_DARK: Color = Color::Rgb(15, 18, 24);
    pub const CARD_BG: Color = Color::Rgb(22, 27, 36);
    pub const TEXT_PRIMARY: Color = Color::Rgb(235, 240, 245);
    pub const TEXT_MUTED: Color = Color::Rgb(110, 125, 140);
    pub const GOLD: Color = Color::Rgb(240, 185, 75);
    pub const CYAN: Color = Color::Rgb(75, 210, 210);
    #[expect(dead_code)]
    pub const GREEN: Color = Color::Rgb(80, 220, 140);
    pub const RED: Color = Color::Rgb(245, 95, 95);
    pub const BG_OVERLAY: Color = Color::Rgb(5, 8, 12);
    pub const BORDER_INACTIVE: Color = Color::Rgb(45, 55, 70);
    pub const BORDER_ACTIVE: Color = Color::Rgb(75, 210, 210);
}

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
        let area = frame.area();
        if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
            Self::draw_min_size_warning(frame, area.width, area.height);
            return;
        }
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

    fn draw_min_size_warning(frame: &mut Frame, w: u16, h: u16) {
        let area = frame.area();

        let block = Block::default()
            .title(" ⚠ Terminal Too Small ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::GOLD));

        let msg = Paragraph::new(vec![
            Line::from(Span::styled(
                format!(" Minimum: {MIN_WIDTH} x {MIN_HEIGHT} "),
                Style::default()
                    .fg(theme::TEXT_PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ))
                .alignment(Alignment::Center),
            Line::from(Span::styled(
                format!(" Current: {w} x {h} "),
                Style::default().fg(theme::RED).add_modifier(Modifier::BOLD),
            ))
                .alignment(Alignment::Center),
            Line::from(""),
            Line::from(Span::styled(
                " Please enlarge the terminal window ",
                Style::default().fg(theme::TEXT_MUTED),
            ))
                .alignment(Alignment::Center),
        ])
            .block(block)
            .alignment(Alignment::Center);

        let centered = Rect {
            x: area.x + area.width.saturating_sub(MIN_WIDTH) / 2,
            y: area.y + area.height.saturating_sub(7) / 2,
            width: MIN_WIDTH.min(area.width),
            height: 7.min(area.height),
        };

        frame.render_widget(Clear, area);
        frame.render_widget(msg, centered);
    }

    fn draw_error(frame: &mut Frame, e: &crate::error::ErrorType) {
        let area = frame.area();

        let err_block = Block::default()
            .title(" ⚠️ Error ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::RED));

        let hint = Line::from(vec![
            Span::styled(
                " [S] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(theme::CYAN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Setup ", Style::default().fg(theme::TEXT_MUTED)),
            Span::styled(" │ ", Style::default().fg(theme::BORDER_INACTIVE)),
            Span::styled(
                " [Q] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(theme::TEXT_PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Quit ", Style::default().fg(theme::TEXT_MUTED)),
        ])
            .alignment(Alignment::Center);

        let msg = Paragraph::new(vec![
            Line::from(Span::styled(
                format!(" {}", e),
                Style::default()
                    .fg(theme::TEXT_PRIMARY)
                    .add_modifier(Modifier::BOLD),
            ))
                .alignment(Alignment::Center),
            Line::from(""),
            hint,
        ])
            .block(err_block)
            .alignment(Alignment::Center);

        let centered = Rect {
            x: area.x + area.width.saturating_sub(56) / 2,
            y: area.y + area.height.saturating_sub(7) / 2,
            width: 56.min(area.width),
            height: 7.min(area.height),
        };

        frame.render_widget(Clear, centered);
        frame.render_widget(msg, centered);
    }
}
