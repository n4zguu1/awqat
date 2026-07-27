use crate::error::ErrorType;
use crate::tui::app::App;

mod core;
mod error;
mod tui;

fn main() -> Result<(), ErrorType> {
    ratatui::run(|terminal| App::new().run(terminal))
}
