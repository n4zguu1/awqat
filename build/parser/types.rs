use crate::core::types::Location;
use salah::{Madhab, Method};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct RawCountries {
    pub countries: Vec<CountryAdhanCalculation>,
}
#[derive(Serialize, Deserialize)]
struct CountryAdhanCalculation {
    pub country: Country,
    #[serde(with = "MadhabDef")]
    pub madhab: Madhab,
    #[serde(with = "MethodDef")]
    pub method: Method,
    pub fajr_angle: u32,
    pub isha_angle: u32,
}
#[derive(Serialize, Deserialize)]
struct Country {
    name: String,
    iso_code: String,
    cities: Vec<City>,
}
#[derive(Serialize, Deserialize)]
struct City {
    name: String,
    coordinates: Location,
}

// shadow helpers
#[derive(Serialize, Deserialize)]
#[serde(remote = "Madhab")]
pub enum MadhabDef {
    Shafi,
    Hanafi,
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
    Tehran,
    Turkey,
    Other,
}
