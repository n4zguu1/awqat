use rusqlite::ToSql;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RawCountries {
    pub countries: Vec<CountryAdhanCalculation>,
}

#[derive(Serialize, Deserialize)]
pub struct CountryAdhanCalculation {
    pub country: Country,
    pub madhab: Madhab,
    #[serde(default)]
    pub method: Method,
    pub fajr_angle: u32,
    pub isha_angle: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Country {
    pub name: String,
    pub iso_code: String,
    #[serde(default)]
    pub cities: Vec<City>,
}

#[derive(Serialize, Deserialize)]
pub struct City {
    pub name: String,
    pub coordinates: Location,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize)]
pub enum Madhab {
    Shafi,
    Hanafi,
}
impl ToSql for Madhab {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let variant_str = match self {
            Madhab::Shafi => "Shafi",
            Madhab::Hanafi => "Hanafi",
        };
        Ok(ToSqlOutput::from(variant_str))
    }
}
#[derive(Serialize, Deserialize, Default)]
pub enum Method {
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
    #[default]
    Other,
}
impl ToSql for Method {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let variant_str = match self {
            Method::MuslimWorldLeague => "MuslimWorldLeague",
            Method::Egyptian => "Egyptian",
            Method::Karachi => "Karachi",
            Method::UmmAlQura => "UmmAlQura",
            Method::Dubai => "Dubai",
            Method::MoonsightingCommittee => "MoonsightingCommittee",
            Method::NorthAmerica => "NorthAmerica",
            Method::Kuwait => "Kuwait",
            Method::Qatar => "Qatar",
            Method::Singapore => "Singapore",
            Method::Tehran => "Tehran",
            Method::Turkey => "Turkey",
            Method::Other => "Other",
        };
        Ok(ToSqlOutput::from(variant_str))
    }
}
