use crate::error::ErrorType;
use crate::tui::app::{App, AppState};
use chrono::Local;
use crossterm::event;
use crossterm::event::KeyCode;
use std::time::Duration;

const DEBOUNCE_MS: u64 = 200;

impl App {
    pub fn handle_event(&mut self) -> Result<(), ErrorType> {
        if event::poll(Duration::from_millis(100))?
            && let Some(key) = event::read()?.as_key_press_event()
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('c')
                if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                    {
                        self.exit = true;
                }
                _ => match &mut self.state {
                    AppState::Running(data) => self.handle_tick(),
                    AppState::Setup(_) => todo!(),
                    AppState::Error(_) => {}
                },
            }
        }
        Ok(())
    }

    pub fn handle_tick(&mut self) {
        if let AppState::Running(data) = &mut self.state {
            data.date_time = Local::now();
        }
    }
}
