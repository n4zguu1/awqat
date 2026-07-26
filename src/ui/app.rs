// data needed to render are
// current time,date and hijri date
// prayer times for today, and this month. i just need to fetch month dates and add logic to preview for today i hve the required data for that
// the country , method of calculation.
// remaining time (calculated)
// time calculations need to be carefully handled, to avoid divergence between user desktop clock time and app shown clock

use crate::core::date::NaiveHijriDate;
use crate::core::storage::{get_config_path, load_config};
use crate::core::time::PrayerTimes;
use crate::core::types::UserData;
use crate::error::ErrorType;
use chrono::{DateTime, Local};

struct RunningData {
    date_time: DateTime<Local>,
    method: String,
    hijri_date: NaiveHijriDate,
    prayer_times: PrayerTimes,
    city: String,
}
impl RunningData {
    fn new(data: &UserData) -> Result<Self, ErrorType> {
        let date_time = Local::now();
        let prayer_times = data.calculate(&date_time.date_naive())?;
        let method = data.country.method.clone().to_string();
        let hijri_date = NaiveHijriDate::from_gregorian_to_ummalqura(&date_time.date_naive())?;
        let city = data.city.name.clone();
        Ok(RunningData {
            date_time,
            method,
            prayer_times,
            city,
            hijri_date,
        })
    }
}
struct SetupData {
    city_input: String,
}
enum AppState {
    Running(RunningData),
    Setup(SetupData),
    Loading,
    Settings,
    Error(ErrorType),
}
struct App {
    state: AppState,
}
impl App {
    // maps all errors to Error state so they can be handled in the UI, instead of return them as result
    fn new() -> Self {
        // check config to determine setup from running
        let file_path = match get_config_path() {
            Ok(path) => path,
            Err(e) => {
                return App {
                    state: AppState::Error(e),
                };
            }
        };
        let state = match load_config::<UserData>(&file_path) {
            Ok(data) => {
                let running_data = match RunningData::new(&data) {
                    Ok(data) => data,
                    Err(e) => {
                        return App {
                            state: AppState::Error(e),
                        };
                    }
                };
                AppState::Running(running_data)
            }
            Err(ErrorType::ConfigFileNotFound) => todo!(),
            Err(e) => AppState::Error(e),
        };
        App { state }
    }
    pub fn run() {}
}
