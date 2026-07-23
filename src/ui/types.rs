use ratatui::Frame;

pub enum Screen {
    Setup,
    Main,
    Settings,
    Search,
    Min // a screen shows when the min dimension is hit
}
struct DayPrayers {
    fajr: String,
    sunrise: String,
    dhuhr: String,
    asr: String,
    maghrib: String,
    isha: String,
}

// the data the application is gonna render (display)
struct App {
    screen: Screen,
    city: String,
    country: String,
    time: String,
    date: String,
    hijri_date: String,
    today_prayers: DayPrayers,
    calendar_prayers: Vec<DayPrayers>,
    method: String,
    exit:bool
}
impl App {
    fn draw (&self, frame: &mut Frame) {
        todo!()
    }
}