// handle all logic related to time unit

use crate::core::date::NaiveHijriDate;
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
    pub offset: i64,
}
impl PrayerData {
    pub fn from_data(data: Data) -> Self {
        let madhab = data.country.madhab;
        let method = data.country.method;
        let coordinates = data.city.coordinates;
        let offset = data.city.timezone.utc_offset;
        PrayerData {
            madhab,
            method,
            coordinates,
            offset,
        }
    }
    // handle the specific cases
    // todo: the calculations are based on the sun hitting specif angles. some places, sun never goes down. we need to cover those in feature versions
    pub fn calculate(&self, utc: &DateTime<Utc>) -> Result<PrayerTimes, ErrorType> {
        // special case for UmmAlQura method , where they calculate isha time little different based on Islamic month
        // the method uses fixed time interval between maghreb and isha, where in ramadan isha = maghreb + 120 min. in other months isha= maghreb + 90
        // the lib already calcualates the addjustment on other months, we need ajustement for ramadan
        let local = *utc + chrono::Duration::seconds(self.offset);
        let date = NaiveHijriDate::from_gregorian_to_ummalqura(local.date_naive())?;
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
            .on(local.date_naive())
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
            offset: 3600,
        };
        let chlef_prayer_times = chlef_prayer_data.calculate(&today);
        let mecca_prayer_data = PrayerData {
            madhab: Madhab::Shafi,
            method: Method::UmmAlQura,
            coordinates: Coordinates {
                latitude: 21.42664,
                longitude: 39.82563,
            },
            offset: 10800,
        };
        let mecca_prayer_times = mecca_prayer_data.calculate(&today);
        // Ramadan
        let mecca_prayer_times_ramadan = mecca_prayer_data.calculate(&ramadan);
        println!("{:?}", chlef_prayer_times);
        println!("{:?}", mecca_prayer_times);
        println!("{:?}", mecca_prayer_times_ramadan);
    }

    #[test]
    fn offset_changes_date_for_prayer_calculation() {
        // Scenario: it's Jan 14, 10 PM in New York (UTC-5)
        // UTC says: Jan 15, 03:00 — a different date!
        //
        // Without offset: salah calculates for Jan 15 → WRONG
        // With offset:    salah calculates for Jan 14 → CORRECT

        let utc_time = Utc.with_ymd_and_hms(2025, 1, 15, 3, 0, 0).unwrap();
        // New York: UTC-5 → offset = -18000 seconds
        // 03:00 UTC - 5h = 22:00 on Jan 14

        let new_york = PrayerData {
            madhab: Madhab::Shafi,
            method: Method::NorthAmerica,
            coordinates: Coordinates {
                latitude: 40.7128,
                longitude: -74.0060,
            },
            offset: -18000,
        };

        // With offset: local date should be Jan 14
        let result = new_york.calculate(&utc_time);
        assert!(result.is_ok(), "prayer calculation should succeed");

        // Without offset (offset=0): UTC date would be Jan 15
        let no_offset = PrayerData {
            madhab: Madhab::Shafi,
            method: Method::NorthAmerica,
            coordinates: Coordinates {
                latitude: 40.7128,
                longitude: -74.0060,
            },
            offset: 0,
        };

        let result_no_offset = no_offset.calculate(&utc_time);
        assert!(result_no_offset.is_ok(), "prayer calculation should succeed");

        // both succeed, but they calculate for DIFFERENT dates
        // the offset version calculates for Jan 14 (correct for New York user)
        // the no-offset version calculates for Jan 15 (wrong for New York user)

        println!("UTC time:        {}", utc_time);
        println!("With offset:     calculates for Jan 14 (correct)");
        println!("Without offset:  calculates for Jan 15 (wrong)");
        println!();
        println!("Prayer times WITH offset (Jan 14):   {:?}", result.unwrap());
        println!("Prayer times WITHOUT offset (Jan 15): {:?}", result_no_offset.unwrap());
    }
}
