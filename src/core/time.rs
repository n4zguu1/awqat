// handle all logic related to time unit

use crate::core::date::HijriDate;
use crate::core::types::MadhabDef;
use crate::core::types::MethodDef;
use crate::core::types::{Coordinates, Data};
use crate::error::ErrorType;
use chrono::{DateTime, Utc};
use salah::{Configuration, Madhab, Method, PrayerSchedule, PrayerTimes, TimeAdjustment};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PrayerData {
    #[serde(with = "MadhabDef")]
    pub madhab: Madhab,
    #[serde(with = "MethodDef")]
    pub method: Method,
    pub coordinates: Coordinates,
}
impl PrayerData {
    pub fn from_data(data: Data) -> Self {
        let madhab = data.country.madhab;
        let method = data.country.method;
        let coordinates = data.city.coordinates;
        PrayerData {
            madhab,
            method,
            coordinates,
        }
    }
    // handle the specific cases
    // todo: the calculations are based on the sun hitting specif angles. some places, sun never goes down. we need to cover those in feature versions
    pub fn calculate_prayer_times(&self, utc: &DateTime<Utc>) -> Result<PrayerTimes, ErrorType> {
        // special case for UmmAlQura method , where they calculate isha time little different based on Islamic month
        // the method uses fixed time interval between maghreb and isha, where in ramadan isha = maghreb + 120 min. in other months isha= maghreb + 90
        // the lib already calcualates the addjustment on other months, we need ajustement for ramadan
        let date = HijriDate::from_gregorian_to_ummalqura(utc.date_naive())?;
        // explicitly adjust for Ramadan
        let params = if date.month == 9 && self.method == Method::UmmAlQura {
            // special case for isha, +30 min then the usual
            let adjustment = TimeAdjustment {
                isha: 30,
                ..TimeAdjustment::default()
            };
            let mut config = Configuration::with(self.method, self.madhab);
            config.adjustments = adjustment;
            config
        } else {
            Configuration::with(self.method, self.madhab)
        };
        let location =
            salah::Coordinates::new(self.coordinates.latitude, self.coordinates.longitude);
        let prayer_times = PrayerSchedule::new()
            .on(utc.date_naive())
            .with_configuration(params)
            .for_location(location)
            .calculate()
            .map_err(ErrorType::CalculatingPrayerTimesFailed)?;
        Ok(prayer_times)
    }
}

#[cfg(test)]
mod test {
    use crate::core::time::PrayerData;
    use crate::core::types::Coordinates;
    use chrono::{TimeZone, Utc};
    use salah::{Madhab, Method};

    #[test]
    fn prayer_times_adjustment() {
        let today = Utc::now();
        let ramadan = Utc.with_ymd_and_hms(2025, 3, 21, 0, 0, 0).unwrap();
        let chlef_prayer_data = PrayerData {
            madhab: Madhab::Shafi,
            method: Method::MuslimWorldLeague,
            coordinates: Coordinates {
                latitude: 36.16525,
                longitude: 1.33452,
            },
        };
        let chlef_prayer_times = chlef_prayer_data.calculate_prayer_times(&today);
        let mecca_prayer_data = PrayerData {
            madhab: Madhab::Shafi,
            method: Method::UmmAlQura,
            coordinates: Coordinates {
                latitude: 21.42664,
                longitude: 39.82563,
            },
        };
        let mecca_prayer_times = mecca_prayer_data.calculate_prayer_times(&today);
        // Ramadan
        let mecca_prayer_times_ramadan = mecca_prayer_data.calculate_prayer_times(&ramadan);
        println!("{:?}", chlef_prayer_times);
        println!("{:?}", mecca_prayer_times);
        println!("{:?}", mecca_prayer_times_ramadan);
    }
}
