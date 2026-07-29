use crate::core::time::time_with_offset;
use crate::error::ErrorType;
use crate::tui::app::{App, AppState, SetupData};
use chrono::Utc;
use crossterm::event;
use crossterm::event::{KeyCode, KeyModifiers, MouseEvent, MouseEventKind};
use std::time::{Duration, Instant};

const SEARCH_DEBOUNCE_MS: u128 = 200;

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
        if self.overlay.is_some() {
            return self.handle_overlay_key(key);
        }

        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Esc => self.exit = true,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => self.exit = true,
            _ => match &mut self.state {
                AppState::Running(data) => match key.code {
                    KeyCode::Char('c') => data.go_to_current(),
                    KeyCode::Char('s') => {
                        self.overlay = Some(SetupData::default());
                    }
                    KeyCode::Up | KeyCode::Char('k') => data.move_cursor(-1)?,
                    KeyCode::Down | KeyCode::Char('j') => data.move_cursor(1)?,
                    KeyCode::Char('g') => data.go_to_top()?,
                    KeyCode::Char('G') => data.go_to_bottom()?,
                    _ => {}
                },
                AppState::Setup(data) => match key.code {
                    KeyCode::Enter => {
                        if let Some(running) = data.select_current()? {
                            self.state = AppState::Running(Box::new(running));
                            self.pending_clear = true;
                        }
                    }
                    _ => Self::handle_setup_key_state(data, key),
                },
                AppState::Error(_) => {
                    if let KeyCode::Char('s') = key.code {
                        self.state = AppState::Setup(SetupData::default());
                        self.pending_clear = true;
                    }
                }
            },
        }
        Ok(())
    }

    fn handle_overlay_key(&mut self, key: event::KeyEvent) -> Result<(), ErrorType> {
        let Some(ref mut overlay) = self.overlay else {
            return Ok(());
        };

        match key.code {
            KeyCode::Esc => {
                self.overlay = None;
            }
            KeyCode::Char(ch) => {
                overlay.query.push(ch);
                overlay.search_triggered = false;
                overlay.last_input = Some(Instant::now());
            }
            KeyCode::Backspace => {
                overlay.query.pop();
                overlay.search_triggered = false;
                overlay.last_input = Some(Instant::now());
            }
            KeyCode::Up => {
                overlay.selected = overlay.selected.saturating_sub(1);
            }
            KeyCode::Down if overlay.selected + 1 < overlay.results.len() => {
                overlay.selected += 1;
            }
            KeyCode::Enter => {
                if let Some(running) = overlay.select_current()? {
                    self.state = AppState::Running(Box::new(running));
                    self.overlay = None;
                    self.pending_clear = true;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_setup_key_state(data: &mut SetupData, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char(ch) => {
                data.query.push(ch);
                data.search_triggered = false;
                data.last_input = Some(Instant::now());
            }
            KeyCode::Backspace => {
                data.query.pop();
                data.search_triggered = false;
                data.last_input = Some(Instant::now());
            }
            KeyCode::Up => {
                data.selected = data.selected.saturating_sub(1);
            }
            KeyCode::Down if data.selected + 1 < data.results.len() => {
                data.selected += 1;
            }
            _ => {}
        }
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) -> Result<(), ErrorType> {
        if self.overlay.is_some() {
            return Ok(());
        }
        match mouse.kind {
            MouseEventKind::ScrollUp => {
                if let AppState::Running(data) = &mut self.state {
                    for _ in 0..3 {
                        data.move_cursor(-1)?;
                    }
                }
            }
            MouseEventKind::ScrollDown => {
                if let AppState::Running(data) = &mut self.state {
                    for _ in 0..3 {
                        data.move_cursor(1)?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn handle_tick(&mut self) {
        if let Some(ref mut overlay) = self.overlay {
            if !overlay.search_triggered
                && let Some(t) = overlay.last_input
                && t.elapsed().as_millis() >= SEARCH_DEBOUNCE_MS
            {
                overlay.search_triggered = true;
                let _ = overlay.search();
            }
            return;
        }

        match &mut self.state {
            AppState::Running(data) => {
                data.date_time = time_with_offset(&data.coordinates, data.utc_offset, Utc::now());
            }
            AppState::Setup(data) => {
                if !data.search_triggered
                    && let Some(t) = data.last_input
                    && t.elapsed().as_millis() >= SEARCH_DEBOUNCE_MS
                {
                    data.search_triggered = true;
                    let _ = data.search();
                }
            }
            AppState::Error(_) => {}
        }
    }
}
