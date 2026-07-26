use crate::ui::app::{App, AppState, RunningData};
use ratatui::Frame;

impl App {
    pub fn draw(&self, frame: &mut Frame) {
        match &self.state {
            AppState::Running(running_data) => running_data.draw(frame),
            AppState::Error(error) => todo!(),
            AppState::Settings => todo!(),
            AppState::Setup(setup_data) => todo!(),
            AppState::Loading => todo!(),
        }
    }
}
impl RunningData {
    pub fn draw(&self, frame: &mut Frame) {}
}
