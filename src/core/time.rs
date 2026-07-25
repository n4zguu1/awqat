// handle all logic related to time unit

use crate::core::date::{HijriMonths, NaiveHijriDate};
use crate::core::types::MadhabDef;
use crate::core::types::MethodDef;
use crate::core::types::{Coordinates, UserData};
use crate::error::ErrorType;
use chrono::{DateTime, Days, Months, NaiveDate, TimeDelta, Utc};
use salah::{
    Configuration, Madhab, Method, Parameters, PrayerSchedule, PrayerTimes, TimeAdjustment,
};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

// sliding window approach, based on user scrolling we pop and push new months

const MONTH_OFFSET: u32 = 4;
// we keep track of last previous and next days to update
pub struct CalendarPrayers {
    first_prev_days: NaiveDate,
    last_next_days: NaiveDate,
    calendar: VecDeque<PrayerTimes>,
}

// Then in your code:
impl CalendarPrayers {
    pub fn from_date(base_date: &NaiveDate, prayer_data: &PrayerData) -> Result<Self, ErrorType> {
        let first_prev_days = base_date
            .checked_sub_months(Months::new(MONTH_OFFSET))
            .unwrap();
        let last_next_days = base_date
            .checked_add_months(Months::new(MONTH_OFFSET))
            .unwrap();
        let mut cal_prayers = VecDeque::new();
        let mut current_day = first_prev_days;
        while current_day <= last_next_days {
            let prayers = prayer_data.calculate(&current_day).map_err(|_e| {
                ErrorType::CalculatingPrayerTimesFailed(
                    "Calculating prayer times failed.".to_string(),
                )
            })?;
            cal_prayers.push_front(prayers);
            current_day += TimeDelta::days(1);
        }
        Ok(CalendarPrayers {
            first_prev_days,
            last_next_days,
            calendar: cal_prayers,
        })
    }

    pub fn pop_first_prev(&mut self) {
        self.calendar.pop_back().unwrap();
    }

    pub fn pop_last_next(&mut self) {
        self.calendar.pop_front().unwrap();
    }
    pub fn push_first_prev(&mut self, prayer_data: &PrayerData) -> Result<(), ErrorType> {
        let new = self.first_prev_days.checked_sub_days(Days::new(1)).unwrap();
        let prayer_times = prayer_data.calculate(&new).map_err(|_e| {
            ErrorType::CalculatingPrayerTimesFailed("Calculating prayer times failed.".to_string())
        })?;
        self.calendar.push_back(prayer_times);
        self.first_prev_days = new;
        Ok(())
    }
    pub fn push_last_next(&mut self, prayer_data: &PrayerData) -> Result<(), ErrorType> {
        let new = self.last_next_days.checked_add_days(Days::new(1)).unwrap();
        let prayer_times = prayer_data.calculate(&new).map_err(|_e| {
            ErrorType::CalculatingPrayerTimesFailed("Calculating prayer times failed.".to_string())
        })?;
        self.calendar.push_front(prayer_times);
        self.last_next_days = new;
        Ok(())
    }
}
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
    pub fn from_data(data: &UserData) -> Self {
        let madhab = data.country.madhab;
        let method = data.country.method;
        let coordinates = data.city.coordinates.clone();
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
    // the calculate isnt aware of location data.
    pub fn calculate(&self, naive_date: &NaiveDate) -> Result<PrayerTimes, ErrorType> {
        // special case for UmmAlQura method , where they calculate isha time little different based on Islamic month
        // the method uses fixed time interval between maghreb and isha, where in ramadan isha = maghreb + 120 min. in other months isha= maghreb + 90
        // the lib already calcualates the addjustment on other months, we need ajustement for ramadan
        let hijri_date = NaiveHijriDate::from_gregorian_to_ummalqura(*naive_date)?;
        // explicitly adjust for Ramadan
        let params = if hijri_date.month == 9 && self.method == Method::UmmAlQura {
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
            .on(*naive_date)
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
    use chrono::{NaiveDate, TimeZone, Utc};
    use salah::{Madhab, Method};

    #[test]
    fn prayer_times_adjustment() {
        let today = Utc::now().date_naive();
        let ramadan = Utc
            .with_ymd_and_hms(2025, 3, 21, 0, 0, 0)
            .unwrap()
            .date_naive();
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
}
