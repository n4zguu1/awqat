use salah::{Madhab, Method};
use serde::{Deserialize, Serialize};

pub static APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub country: Country,
    pub region: Region,
    pub city: City,
}
#[derive(Serialize, Deserialize)]

pub struct Country {
    pub iso2: String, // unique id
    pub name: String,
    #[serde(with = "MadhabDef")]
    pub madhab: Madhab,
    #[serde(with = "MethodDef")]
    pub method: Method,
}
#[derive(Serialize, Deserialize)]
#[serde(remote = "Madhab")]
pub enum MadhabDef {
    Hanafi,
    Shafi,
}
#[derive(Serialize, Deserialize)]
#[serde(remote = "Method")]
pub enum MethodDef {
    MuslimWorldLeague,
    Egyptian,
    Karachi,
    UmmAlQura,
    Dubai,
    MoonsightingCommittee,
    NorthAmerica,
    Kuwait,
    Qatar,
    Singapore,
    Turkey,
    Tehran,
    Other,
}
impl Country {
    pub fn new(iso2: String, name: String, madhab: Madhab, method: Method) -> Self {
        Self {
            iso2,
            name,
            madhab,
            method,
        }
    }
}
#[derive(Serialize, Deserialize)]

pub struct Region {
    pub name: String,
}

impl Region {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[derive(Serialize, Deserialize)]

pub struct City {
    pub name: String,
    pub coordinates: Coordinates,
    pub timezone: Timezone,
}

impl City {
    pub fn new(name: String, coordinates: Coordinates, timezone: Timezone) -> Self {
        Self {
            name,
            coordinates,
            timezone,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinates {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct Timezone {
    utc_offset: i64,
}
impl Timezone {
    pub fn new(offset: i64) -> Self {
        Timezone { utc_offset: offset }
    }
    pub fn to_utc_format(&self) -> String {
        let is_negative = self.utc_offset < 0;
        let abs_offset_seconds = self.utc_offset.abs();

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
mod tests {
    use crate::core::types::Timezone;
    #[test]
    pub fn gmt_name() {
        let timezone = Timezone::new(-21600);
        let timezone2 = Timezone::new(16200);
        assert_eq!(timezone.to_utc_format(), "UTC-06:00".to_string());
        assert_eq!(timezone2.to_utc_format(), "UTC+04:30".to_string())
    }
}
