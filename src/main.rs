use crate::core::init_core;
use crate::error::ErrorType;

mod core;
mod error;
mod tui;

fn main() -> Result<(), ErrorType> {
    init_core();
    Ok(())
    //  ratatui::run(|terminal| App::new().run(terminal))
}
