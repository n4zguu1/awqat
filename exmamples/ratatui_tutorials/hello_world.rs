use crate::error::ErrorType;
use ratatui::{DefaultTerminal, Frame};


pub fn hello_world() -> Result<(), ErrorType> {
    // this helps us display error in nice formatings
    color_eyre::install().map_err(ErrorType::ColorEyreOperationFailed)?;
    // main entry point , where we call our run function to run event loop
    ratatui::run(app).map_err(ErrorType::IOError)?;
    Ok(())
}

// this is the app event loop
// we draw things on it
// we tell the backend if any key pressed return true
// if true, the even loop breaks, the program ends
fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

// this is the drawing functin
// we simple use primitve, which is &str, and define where to put that string. which is the frame area . no layouts , no complex shit
fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area())
}
