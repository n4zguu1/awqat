// handle all logic related to time unit

use crate::core::date::NaiveHijriDate;
use std::collections::VecDeque;

use crate::core::types::{Method, UserData};
use crate::error::ErrorType;
use chrono::{DateTime, Datelike, Days, Local, Month, Months, NaiveDate, Utc};
use salah::Prayer::{Asr, Dhuhr, Fajr, Isha, Maghrib, Sunrise};
use salah::{Configuration, PrayerSchedule, TimeAdjustment};
use serde::{Deserialize, Serialize};

// preload should be even for keep the anchor month in center and better UX
const MONTHS_PRELOAD: u32 = 4;

#[allow(dead_code)]
pub struct BatchPrayers {
    base: NaiveDate,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DayPrayerTimes {
    pub date: NaiveDate,
    pub hijri: NaiveHijriDate,
    pub fajr: DateTime<Utc>,
    pub sunrise: DateTime<Utc>,
    pub dhuhr: DateTime<Utc>,
    pub asr: DateTime<Utc>,
    pub maghrib: DateTime<Utc>,
    pub isha: DateTime<Utc>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MonthPrayerTimes {
    pub days: Vec<DayPrayerTimes>,
    pub date: NaiveDate,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CalendarPrayerTimes {
    pub months: VecDeque<MonthPrayerTimes>,
}

impl DayPrayerTimes {
    pub fn as_string_with_timezone(&self) -> [String; 8] {
        let date = self.date.to_string();
        let hijri = self.hijri.to_string();
        let fajr = self
            .fajr
            .with_timezone(&Local)
            .format("%I:%M %p")
            .to_string();
        let sunrise = self
            .sunrise
            .with_timezone(&Local)
            .format("%I:%M %p")
            .to_string();
        let dhuhr = self
            .dhuhr
            .with_timezone(&Local)
            .format("%I:%M %p")
            .to_string();
        let asr = self
            .asr
            .with_timezone(&Local)
            .format("%I:%M %p")
            .to_string();
        let maghrib = self
            .maghrib
            .with_timezone(&Local)
            .format("%I:%M %p")
            .to_string();
        let isha = self
            .isha
            .with_timezone(&Local)
            .format("%I:%M %p")
            .to_string();

        [date, hijri, fajr, sunrise, dhuhr, asr, maghrib, isha]
    }
    #[allow(dead_code)]
    pub fn remaining(&self) -> String {
        todo!()
    }
}

impl UserData {
    // handle the specific cases
    // todo: the calculations are based on the sun hitting specif angles. some places, sun never goes down. we need to cover those in feature versions
    // the calculate isnt aware of location data.
    pub fn calculate(&self, naive_date: &NaiveDate) -> Result<DayPrayerTimes, ErrorType> {
        // special case for UmmAlQura method , where they calculate isha time little different based on Islamic month
        // the method uses fixed time interval between maghreb and isha, where in ramadan isha = maghreb + 120 min. in other months isha= maghreb + 90
        // the lib already calcualates the addjustment on other months, we need ajustement for ramadan
        let hijri_date = NaiveHijriDate::from_gregorian_to_ummalqura(naive_date)?;
        // explicitly adjust for Ramadan
        let params = if hijri_date.month == 9 && self.country.method == Method::UmmAlQura {
            // special case for isha, +30 min then the usual
            let adjustment = TimeAdjustment {
                isha: 30,
                ..TimeAdjustment::default()
            };
            let mut config = Configuration::with(
                self.country.method.to_crate(),
                self.country.madhab.to_crate(),
            );
            config.adjustments = adjustment;
            config
        } else {
            Configuration::with(
                self.country.method.to_crate(),
                self.country.madhab.to_crate(),
            )
        };
        let location = salah::Coordinates::new(
            self.city.coordinates.latitude,
            self.city.coordinates.longitude,
        );
        let prayers = PrayerSchedule::new()
            .on(*naive_date)
            .with_configuration(params)
            .for_location(location)
            .calculate()
            .map_err(ErrorType::CalculatingPrayerTimesFailed)?;
        let fajr = prayers.time(Fajr);
        let sunrise = prayers.time(Sunrise);
        let dhuhr = prayers.time(Dhuhr);
        let asr = prayers.time(Asr);
        let maghrib = prayers.time(Maghrib);
        let isha = prayers.time(Isha);

        Ok(DayPrayerTimes {
            date: *naive_date,
            hijri: hijri_date,
            fajr,
            sunrise,
            dhuhr,
            asr,
            maghrib,
            isha,
        })
    }
    // the key idea is to to presume a number of months to start with
    pub fn calculate_month(&self, date: &NaiveDate) -> Result<MonthPrayerTimes, ErrorType> {
        let month = Month::try_from(date.month() as u8).unwrap();
        let days_nbr = month.num_days(date.year()).unwrap();
        let mut days = Vec::with_capacity(days_nbr as usize);

        let mut current = if let Some(date) = NaiveDate::from_ymd_opt(date.year(), date.month(), 1)
        {
            date
        } else {
            return Err(ErrorType::DateNotFound);
        };
        for _ in 1..=days_nbr {
            let prayer_times = self.calculate(&current)?;
            days.push(prayer_times);
            current = current.checked_add_days(Days::new(1)).unwrap()
        }
        Ok(MonthPrayerTimes { days, date: *date })
    }
    // we presume a number of months, we can play with two ends popping and pushing
    // the output is sorted
    pub fn calculate_batch(&self, date: &NaiveDate) -> Result<CalendarPrayerTimes, ErrorType> {
        let mut calendar = VecDeque::with_capacity(MONTHS_PRELOAD as usize);
        let first_preload_date =
            if let Some(date) = date.checked_sub_months(Months::new(MONTHS_PRELOAD / 2)) {
                date
            } else {
                return Err(ErrorType::DateNotFound);
            };
        let mut current_date = first_preload_date;
        let d = MONTHS_PRELOAD / 2;
        for i in 0..(MONTHS_PRELOAD + 1) {
            let month_prayers = if let Ok(prayers) = self.calculate_month(&current_date) {
                prayers
            } else {
                return Err(ErrorType::PrayerCalculationFailed);
            };
            calendar.push_front(month_prayers);

            current_date = if let Some(date) = current_date.checked_add_months(Months::new(1)) {
                date
            } else {
                return Err(ErrorType::DateNotFound);
            }
        }
        Ok(CalendarPrayerTimes { months: calendar })
    }
    // popping from front of vector and pushing to back
    pub fn scroll_down(
        &self,
        calendar_prayer_times: &mut CalendarPrayerTimes,
    ) -> Result<(), ErrorType> {
        if calendar_prayer_times.months.is_empty() {
            return Err(ErrorType::PrayerCalculationFailed);
        }
        let last_month_prayers = calendar_prayer_times
            .months
            .back()
            .expect("vector should always have at least two slots");
        // we catch cases if month is January
        let new_date =
            if let Some(date) = last_month_prayers.date.checked_sub_months(Months::new(1)) {
                date
            } else {
                return Err(ErrorType::DateNotFound);
            };
        let new_date_prayers = self.calculate_month(&new_date)?;
        calendar_prayer_times.months.push_back(new_date_prayers);
        calendar_prayer_times.months.pop_front();
        Ok(())
    }
    pub fn scroll_up(
        &self,
        calendar_prayer_times: &mut CalendarPrayerTimes,
    ) -> Result<(), ErrorType> {
        if calendar_prayer_times.months.is_empty() {
            return Err(ErrorType::PrayerCalculationFailed);
        }
        let last_month_prayers = calendar_prayer_times
            .months
            .front()
            .expect("vector should always have at least two slots");
        // we catch cases if month is January
        let new_date =
            if let Some(date) = last_month_prayers.date.checked_add_months(Months::new(1)) {
                date
            } else {
                return Err(ErrorType::DateNotFound);
            };
        let new_date_prayers = self.calculate_month(&new_date)?;
        calendar_prayer_times.months.push_front(new_date_prayers);
        calendar_prayer_times.months.pop_back();
        Ok(())
    }
    #[allow(dead_code)]
    pub fn calculate_with_angles() {}
}
