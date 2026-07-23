use crate::error::ErrorType;
use ratatui::{DefaultTerminal, Frame};

mod core;
pub mod error;
mod ui;

fn main() -> Result<(), ErrorType> {
    color_eyre::install().map_err(ErrorType::ColorEyreInstallationFailed)?;
    ratatui::run(app).map_err(ErrorType::IOError)?;
    Ok(())
}
fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}
fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area())
}
