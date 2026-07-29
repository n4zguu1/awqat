use crate::tui::app::{App, AppState};
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};

pub mod running;
pub mod setup;

mod theme {
    use ratatui::style::Color;

    pub const BG_DARK: Color = Color::Rgb(15, 18, 24);
    pub const CARD_BG: Color = Color::Rgb(22, 27, 36);
    pub const TEXT_PRIMARY: Color = Color::Rgb(235, 240, 245);
    pub const TEXT_MUTED: Color = Color::Rgb(110, 125, 140);
    pub const GOLD: Color = Color::Rgb(240, 185, 75);
    pub const CYAN: Color = Color::Rgb(75, 210, 210);
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

    fn draw_error(frame: &mut Frame, _e: &crate::error::ErrorType) {
        let area = frame.area();
        let err_block = Block::default()
            .title(" ⚠️ Error ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::RED));

        let msg = Paragraph::new("An unexpected error occurred. Please restart the application.")
            .block(err_block)
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme::TEXT_PRIMARY));

        let centered = Rect {
            x: area.x + area.width.saturating_sub(50) / 2,
            y: area.y + area.height.saturating_sub(5) / 2,
            width: 50.min(area.width),
            height: 5.min(area.height),
        };

        frame.render_widget(Clear, centered);
        frame.render_widget(msg, centered);
    }
}
