use crate::error::ErrorType;
use crate::ratatui_tutorials::counter_app::counter_app;
use crate::ratatui_tutorials::hello_world::hello_world;

mod core;
pub mod error;
mod ui;

mod ratatui_tutorials;

fn main() -> Result<(), ErrorType> {
    // testing out some tutorials
    // hello_world()?;
    counter_app()?;
    Ok(())
}
