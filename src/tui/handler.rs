use crate::error::ErrorType;
use crate::tui::app::{App, AppState};
use chrono::Local;
use crossterm::event;
use crossterm::event::KeyCode;
use std::time::Duration;

impl App {
    pub fn handle_event(&mut self) -> Result<(), ErrorType> {
        if event::poll(Duration::from_millis(100))? {
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') => self.exit = true,
                    _ => {}
                }
            }
        }
        Ok(())
    }
    pub fn handle_tick(&mut self) {
        match &mut self.state {
            AppState::Running(data) => {
                data.date_time = Local::now();
            }
            _ => {}
        }
    }
}
