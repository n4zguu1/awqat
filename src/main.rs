use crate::core::init_core;
use crate::error::ErrorType;
use crate::tui::app::App;

mod core;
mod error;
mod tui;

fn main() -> Result<(), ErrorType> {
    init_core();

    ratatui::run(|terminal| App::new().run(terminal))
}
