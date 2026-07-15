use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use thiserror::Error;

pub static DATA_BASE_PATH: &str = "data/data.db";
pub static APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Error)]
pub enum ErrorType {
    #[error("The requested date was not found")]
    DateNotFound,
    #[error("Error calculating the prayer time")]
    PrayerCalculationFailed,
    #[error("City was not found")]
    CityNotFound,
    #[error("Creating temp directory failed")]
    CreateTmpDirFailed(std::io::Error),
    #[error("Creating temp file failed")]
    CreateTmpFileFailed(std::io::Error),
    #[error("Writing to temp file failed")]
    WriteTmpFileFailed(std::io::Error),
    #[error("Flushing the writer failed")]
    FlushFailed(std::io::Error),
    #[error("Opening the DB connection failed")]
    ConnectionOpenFailed(rusqlite::Error),
}
#[derive(Serialize, Deserialize)]
pub struct DayPrayerEntries {
    pub fajr: DateTime<Utc>,
    pub sunrise: DateTime<Utc>,
    pub dhuhr: DateTime<Utc>,
    pub asr: DateTime<Utc>,
    pub maghrib: DateTime<Utc>,
    pub isha: DateTime<Utc>,
}
impl Display for DayPrayerEntries {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
#[derive(Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}
impl Location {}
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}
pub enum Meridiem {
    AM,
    PM,
}
pub struct Config;
