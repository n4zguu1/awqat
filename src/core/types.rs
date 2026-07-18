#![allow(warnings)]

use salah::{Madhab, Method};
use serde::{Deserialize, Serialize};

pub static APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Data {
    pub country: Country,
    pub region: Region,
    pub city: City,
}
pub struct Country {
    pub iso2: String, // unique id
    pub name: String,
    pub native: String,
    pub madhab: Madhab,
    pub method: Method,
    pub angles: Angles,
}
pub struct Region {
    pub name: String,
}
pub struct City {
    pub name: String,
    pub coordinates: Coordinates,
    pub timezone: Timezone,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(PartialEq)]
pub struct Angles {
    pub fajr: f64,
    pub isha: f64,
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
    pub fn to_gmt_format(&self) -> String {
        let is_negative = self.gmt_offset < 0;
        let abs_offset_seconds = self.gmt_offset.abs();

        // 2. Do clean integer math instead of risking float precision issues
        let hours = abs_offset_seconds / 3600;
        let minutes = (abs_offset_seconds % 3600) / 60;

        let stf = format!("{:02}:{:02}", hours, minutes);

        if is_negative {
            format!("UTC-{}", stf)
        } else {
            format!("UTC+{}", stf)
        }
    }
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

#[cfg(test)]
pub mod tests {
    use crate::core::types::Timezone;
    #[test]
    pub fn gmt_name() {
        let timezone = Timezone::new("Central Standard Time (North America", -21600);
        let timezone2 = Timezone::new("Central Standard Time (North America", 16200);
        assert_eq!(timezone.to_gmt_format(), "UTC-06:00".to_string());
        assert_eq!(timezone2.to_gmt_format(), "UTC+04:30".to_string())
    }
}
