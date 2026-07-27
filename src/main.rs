use crate::core::init_core;
use crate::tui::app::App;

mod core;
mod error;
mod tui;

fn main() {
    init_core();
    ratatui::run(|terminal| App::new().run(terminal)).unwrap();
}
