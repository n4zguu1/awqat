use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize)]
pub struct DayEntries {
    fajr: DateTime<Utc>,
    sunrise: DateTime<Utc>,
    dhuhr: DateTime<Utc>,
    asr: DateTime<Utc>,
    maghreb: DateTime<Utc>,
    isha: DateTime<Utc>,
}
impl Display for DayEntries {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
pub struct Location {
    latitude: f64,
    longitude: f64,
}
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}
pub enum Meridiem {
    AM,
    PM
}
