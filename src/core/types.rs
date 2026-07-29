use salah::{Madhab as MadhabCrate, Method as MethodCrate};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub country: Country,
    pub region: Region,
    pub city: City,
}

#[derive(Serialize, Deserialize)]

pub struct Country {
    pub iso2: String, // unique id
    pub name: String,
    pub madhab: Madhab,
    pub method: Method,
}
#[derive(Serialize, Deserialize)]
pub enum Madhab {
    Hanafi,
    Shafi,
}
impl Madhab {
    pub fn from_crate(m: &MadhabCrate) -> Self {
        match m {
            MadhabCrate::Hanafi => Madhab::Hanafi,
            MadhabCrate::Shafi => Madhab::Shafi,
        }
    }
    pub fn to_crate(&self) -> MadhabCrate {
        match self {
            Madhab::Hanafi => MadhabCrate::Hanafi,
            Madhab::Shafi => MadhabCrate::Shafi,
        }
    }
}
#[derive(Serialize, Deserialize, PartialEq, Clone)]
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
    Turkey,
    Tehran,
    Other,
}
impl Method {
    fn from_crate(m: &MethodCrate) -> Self {
        match m {
            MethodCrate::MuslimWorldLeague => Method::MuslimWorldLeague,
            MethodCrate::Egyptian => Method::Egyptian,
            MethodCrate::Karachi => Method::Karachi,
            MethodCrate::UmmAlQura => Method::UmmAlQura,
            MethodCrate::Dubai => Method::Dubai,
            MethodCrate::MoonsightingCommittee => Method::MoonsightingCommittee,
            MethodCrate::NorthAmerica => Method::NorthAmerica,
            MethodCrate::Kuwait => Method::Kuwait,
            MethodCrate::Qatar => Method::Qatar,
            MethodCrate::Singapore => Method::Singapore,
            MethodCrate::Turkey => Method::Turkey,
            MethodCrate::Tehran => Method::Tehran,
            MethodCrate::Other => Method::Other,
        }
    }
    pub(crate) fn to_crate(&self) -> MethodCrate {
        match self {
            Method::MuslimWorldLeague => MethodCrate::MuslimWorldLeague,
            Method::Egyptian => MethodCrate::Egyptian,
            Method::Karachi => MethodCrate::Karachi,
            Method::UmmAlQura => MethodCrate::UmmAlQura,
            Method::Dubai => MethodCrate::Dubai,
            Method::MoonsightingCommittee => MethodCrate::MoonsightingCommittee,
            Method::NorthAmerica => MethodCrate::NorthAmerica,
            Method::Kuwait => MethodCrate::Kuwait,
            Method::Qatar => MethodCrate::Qatar,
            Method::Singapore => MethodCrate::Singapore,
            Method::Turkey => MethodCrate::Turkey,
            Method::Tehran => MethodCrate::Tehran,
            Method::Other => MethodCrate::Other,
        }
    }
}
impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
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
            Method::Turkey => "Turkey",
            Method::Tehran => "Tehran",
            Method::Other => "Other",
        };

        write!(f, "{name}")
    }
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

#[derive(Serialize, Deserialize, PartialEq, Clone)]
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
    pub utc_offset: i64,
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

impl UserData {
    pub fn new(country: Country, region: Region, city: City) -> Self {
        UserData {
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
