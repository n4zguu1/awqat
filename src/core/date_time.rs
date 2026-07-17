use crate::core::error::ErrorType;
use crate::core::types::{Angles, Data};
use chrono::{DateTime, Utc};
use salah::{Configuration, Coordinates, Madhab, Method, Prayer, PrayerSchedule};
use serde::{Deserialize, Serialize};

// TODO: needs more handling, like some times have time gaps like isha = maghreb + 30min , ishaa = maghreb + 90 min under ramadan month
pub fn calculate_prayer_times(
    date_time: DateTime<Utc>,
    params: MetaData,
) -> Result<DayPrayerEntries, ErrorType> {
    let city = Coordinates::new(params.coordinates.latitude, params.coordinates.longitude);
    let params = if params.method == Method::Other {
        let fajr = params.angles.fajr.unwrap_or(0.0);
        let isha = params.angles.isha.unwrap_or(0.0);
        Configuration::new(fajr, isha).done()
    } else {
        Configuration::with(params.method, params.madhab)
    };
    let date = date_time.date_naive();
    let calculate = PrayerSchedule::new()
        .on(date)
        .for_location(city)
        .with_configuration(params)
        .calculate();
    let prayer = if let Ok(prayer) = calculate {
        prayer
    } else {
        return Err(ErrorType::PrayerCalculationFailed);
    };
    let fajr = prayer.time(Prayer::Fajr);
    let sunrise = prayer.time(Prayer::Sunrise);
    let dhuhr = prayer.time(Prayer::Dhuhr);
    let asr = prayer.time(Prayer::Asr);
    let maghrib = prayer.time(Prayer::Maghrib);
    let isha = prayer.time(Prayer::Isha);
    Ok(DayPrayerEntries {
        fajr,
        sunrise,
        dhuhr,
        asr,
        maghrib,
        isha,
    })
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
pub struct MetaData {
    pub madhab: Madhab,
    pub method: Method,
    pub angles: Angles,
    pub coordinates: crate::core::types::Coordinates,
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
    pub fn calculate_prayers_time(&self, utc: &DateTime<Utc>) -> DayPrayerEntries {

    }
}

#[cfg(test)]
mod test {
    use crate::core::date_time::calculate_prayer_times;
    use salah::{Madhab, Method};

    fn test_calculate_prayer_times() {}
}
