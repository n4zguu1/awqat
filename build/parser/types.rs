use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AwqatData {
    pub countries: Vec<Country>,
}

#[derive(Serialize, Deserialize)]
pub struct Country {
    pub iso2: String,
    pub name: String,
    pub native: Option<String>,
    pub method: String,
    pub angles: Angles,
    pub region: Vec<Region>
}

#[derive(Serialize, Deserialize)]
pub struct Angles {
    pub fajr: Option<f64>,
    pub isha: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct Region {
    pub name: String,
    pub city: Vec<City>
}

#[derive(Serialize, Deserialize)]
pub struct City {
    pub name: String,
    pub coordinates: Coordinates,
    pub timezone: Timezone,
}

#[derive(Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Timezone {
    pub tz_name: String,
    pub gmt_offset: i64,
}
