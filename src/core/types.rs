use chrono::{DateTime, Utc};
use salah::{Madhab, Method};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
    #[error("Rusqlite Operation failed")]
    SqliteOperationFailed(rusqlite::Error),
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
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

// AM/PM based
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
    meridiem: Meridiem,
}
pub enum Meridiem {
    AM,
    PM,
}
#[derive(PartialEq)]
pub struct Angles {
    pub fajr: Option<f64>,
    pub isha: Option<f64>,
}
pub struct City {
    pub name: String,
    pub coordinates: Coordinates,
    pub timezone: Timezone,
}
pub struct Timezone {
    tz_name: String, // Atlantic Standard Time
    gmt_offset: i64,
}
impl Timezone {
    pub fn new(name: &str, offset: i64) -> Self {
        Timezone {
            tz_name: name.to_string(),
            gmt_offset: offset,
        }
    }
    pub fn gmt_name(&self) -> String {
        // 1. Determine the sign and work entirely with the absolute (positive) offset
        let is_negative = self.gmt_offset < 0;
        let abs_offset_seconds = self.gmt_offset.abs();

        // 2. Do clean integer math instead of risking float precision issues
        let hours = abs_offset_seconds / 3600;
        let minutes = (abs_offset_seconds % 3600) / 60;

        // 3. Format the time block (always positive)
        let stf = format!("{:02}:{:02}", hours, minutes);

        // 4. Prepend the correct sign
        if is_negative {
            format!("UTC-{}", stf)
        } else {
            format!("UTC+{}", stf)
        }
    }
}
pub struct Region {
    pub name: String,
}
pub struct Country {
    pub iso2: String, // unique id
    pub name: String,
    pub native: String,
    pub madhab: Madhab,
    pub method: Method,
    pub angles: Angles,
}

pub struct Data {
    pub country: Country,
    pub region: Region,
    pub city: City,
}
impl Data {
    pub fn new(country: Country, region: Region, city: City) -> Self {
        Data {
            country,
            region,
            city,
        }
    }
}
pub struct MetaData {
    pub madhab: Madhab,
    pub method: Method,
    pub angles: Angles,
    pub coordinates: Coordinates,
}
impl MetaData {
    pub fn new(data: Data) -> Self {
        let madhab = data.country.madhab;
        let method = data.country.method;
        let angles = data.country.angles;
        let coordinates = data.city.coordinates;
        MetaData {
            madhab,
            method,
            angles,
            coordinates,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::core::types::Timezone;
    #[test]
    pub fn test_gmt_name() {
        let timezone = Timezone::new("Central Standard Time (North America", -21600);
        let timezone2 = Timezone::new("Central Standard Time (North America", 16200);
        assert_eq!(timezone.gmt_name(), "UTC-06:00".to_string());
        assert_eq!(timezone2.gmt_name(), "UTC+04:30".to_string())
    }
}
