use crate::error::ErrorType;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Text, Widget};
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph};
use ratatui::{DefaultTerminal, Frame};

// init the terminal
// run the app loop
// restore the app to its original state
pub fn counter_app() -> Result<(), ErrorType> {
    color_eyre::install().map_err(ErrorType::ColorEyreOperationFailed)?;
    // deprecated
    // let mut terminal = ratatui::init();
    // App::default().run(&mut terminal)?;
    ratatui::run(|terminal| App::default().run(terminal))?;
    ratatui::restore();

    Ok(())
}
#[derive(Default)]
struct App {
    counter: u8,
    exit: bool,
}
impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), ErrorType> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    fn handle_events(&mut self) -> Result<(), ErrorType> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Right => self.increment_counter(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }
    fn increment_counter(&mut self) {
        self.counter += 1;
    }
    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
    fn exit(&mut self) {
        self.exit = true;
    }
}
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from("Counter App".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let text = Text::from(Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ]));
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
