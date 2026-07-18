// handle all logic related to time unit

use crate::core::date::HijriDate;
use crate::core::error::ErrorType;
use crate::core::types::{Angles, Data};
use chrono::{DateTime, Utc};
use salah::{
    Configuration, Coordinates, Madhab, Method, PrayerSchedule, PrayerTimes, TimeAdjustment,
};

pub struct PrayerData {
    pub madhab: Madhab,
    pub method: Method,
    pub coordinates: crate::core::types::Coordinates,
}
impl PrayerData {
    pub fn new(data: Data) -> Self {
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
    // todo: the calculations are based on the sun hitting specif angles, some places sun never goes down. we need to cover those in feature versions
    pub fn calculate_prayer_times(&self, utc: &DateTime<Utc>) -> Result<PrayerTimes, ErrorType> {
        // special case for UmmAlQura method , where they calculate isha time little different based on Islamic month
        // the method uses fixed time interval between maghreb and isha, where in ramadan isha = maghreb + 120 min. in other months isha= maghreb + 90
        // the lib already calcualates the addjustment on other months, we need ajustement for ramadan
        let date = HijriDate::from_gregorian_to_ummalqura(utc.date_naive())?;
        // explicitly adjust for Ramadan
        let params = if date.ordinal == 9 && self.method == Method::UmmAlQura {
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
        let location = Coordinates::new(self.coordinates.latitude, self.coordinates.longitude);
        let prayer_times = PrayerSchedule::new()
            .on(utc.date_naive())
            .with_configuration(params)
            .for_location(location)
            .calculate()
            .map_err(ErrorType::CalculatingPrayerTimesFailed)?;
        Ok(prayer_times)
    }
}
enum DisplayMode {
    All,
    ImportantPrayers,
    ImportantPrayersWithSunrise,
}

#[cfg(test)]
mod test {
    use crate::core::time::PrayerData;
    use crate::core::types::Coordinates;
    use chrono::{NaiveDate, TimeZone, Utc};
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
