use crate::error::ErrorType;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Paragraph};
use ratatui::{DefaultTerminal, Frame};

// init the terminal
// run the app loop
// restore the app to its original state
pub fn counter_app() -> Result<(), ErrorType> {
    // the run app
    // this function initiate the terminal, enables raw mode and do other stuff,  accepts terminals as param
    ratatui::run(|terminal| App::default().run(terminal))
}
#[derive(Debug, Default)]
// Default attributes assigne default values to fields, like exit to false, counter to zero
// the struct define the states that gonna change over frames
pub struct App {
    counter: u8,
    exit: bool,
}
impl App {
    // the event loop with it we decide the logic of our app
    // accepts terminal as its params
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), ErrorType> {
        // loop
        while !self.exit {
            terminal
                .draw(|frame| self.draw(frame))
                .map_err(ErrorType::IOError)?;
            self.handle_event()?;
        }
        Ok(())
    }
    // what to draw in the frame, we implemeted a widget trait ,
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }
    fn handle_event(&mut self) -> Result<(), ErrorType> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn exit(&mut self) {
        self.exit = true
    }
}
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title)
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);
        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf)
    }
}
