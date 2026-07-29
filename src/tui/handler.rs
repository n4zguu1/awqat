use crate::core::time::time_with_offset;
use crate::error::ErrorType;
use crate::tui::app::{App, AppState};
use chrono::Utc;
use crossterm::event;
use crossterm::event::{KeyCode, MouseEvent};
use std::time::Duration;

impl App {
    pub fn handle_event(&mut self) -> Result<(), ErrorType> {
        if event::poll(Duration::from_millis(200))? {
            let ev = event::read()?;
            match ev {
                event::Event::Key(key) => self.handle_key(key)?,
                event::Event::Mouse(mouse) => self.handle_mouse(mouse)?,
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: event::KeyEvent) -> Result<(), ErrorType> {
        match key.code {
            KeyCode::Char('q') => {
                self.exit = true;
            }
            KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                self.exit = true;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) -> Result<(), ErrorType> {
        Ok(())
    }

    pub fn handle_tick(&mut self) {
        if let AppState::Running(data) = &mut self.state {
            data.date_time = time_with_offset(&data.coordinates, data.utc_offset, Utc::now());
        }
    }
}
