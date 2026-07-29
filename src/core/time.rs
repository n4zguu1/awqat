// handle all logic related to time unit

use crate::core::date::NaiveHijriDate;
use crate::core::types::{Coordinates, Method, UserData};
use crate::error::ErrorType;
use chrono::{DateTime, Datelike, Days, Month, Months, NaiveDate, TimeDelta, Utc};
use chrono_tz::Tz;
use salah::Prayer::{Asr, Dhuhr, Fajr, FajrTomorrow, Isha, Maghrib, Sunrise};
use salah::{Configuration, PrayerSchedule, TimeAdjustment};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

// preload should be even for keep the anchor month in center and better UX
const MONTHS_PRELOAD: u32 = 4;

#[derive(Debug)]
pub enum DayPrayers {
    Fajr,
    Sunrise,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
    FajrTomorrow,
}
impl Display for DayPrayers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            DayPrayers::Fajr => "Fajr",
            DayPrayers::Sunrise => "Sunrise",
            DayPrayers::Dhuhr => "Dhuhr",
            DayPrayers::Asr => "Asr",
            DayPrayers::Maghrib => "Maghrib",
            DayPrayers::Isha => "Isha",
            DayPrayers::FajrTomorrow => "Fajr Tomorrow",
        };
        f.write_str(name)
    }
}
#[derive(Debug)]
pub struct NextPrayer {
    pub time: DateTime<Tz>,
    pub prayer: DayPrayers,
    pub remaining: TimeDelta,
}
#[derive(Clone, Debug)]
pub struct DayPrayerTimes {
    pub date: NaiveDate,
    pub hijri: NaiveHijriDate,
    pub fajr: DateTime<Tz>,
    pub sunrise: DateTime<Tz>,
    pub dhuhr: DateTime<Tz>,
    pub asr: DateTime<Tz>,
    pub maghrib: DateTime<Tz>,
    pub isha: DateTime<Tz>,
    pub fajr_tomorrow: DateTime<Tz>,
}
impl DayPrayerTimes {
    fn time(&self, prayer: &DayPrayers) -> DateTime<Tz> {
        match prayer {
            DayPrayers::Fajr => self.fajr,
            DayPrayers::Sunrise => self.sunrise,
            DayPrayers::Dhuhr => self.dhuhr,
            DayPrayers::Asr => self.asr,
            DayPrayers::Maghrib => self.maghrib,
            DayPrayers::Isha => self.isha,
            DayPrayers::FajrTomorrow => self.fajr_tomorrow,
        }
    }

    pub fn next_prayer(&self, time: &DateTime<Tz>) -> NextPrayer {
        let next = if self.time(&DayPrayers::Fajr) >= *time {
            DayPrayers::Fajr
        } else if self.time(&DayPrayers::Sunrise) >= *time {
            DayPrayers::Sunrise
        } else if self.time(&DayPrayers::Dhuhr) >= *time {
            DayPrayers::Dhuhr
        } else if self.time(&DayPrayers::Asr) >= *time {
            DayPrayers::Asr
        } else if self.time(&DayPrayers::Maghrib) >= *time {
            DayPrayers::Maghrib
        } else if self.time(&DayPrayers::Isha) >= *time {
            DayPrayers::Isha
        } else if self.time(&DayPrayers::FajrTomorrow) >= *time {
            DayPrayers::FajrTomorrow
        } else {
            panic!("failed to calculate next prayer time");
        };
        let next_prayer_time = self.time(&next);
        let remaining = *time - next_prayer_time;
        NextPrayer {
            prayer: next,
            time: next_prayer_time,
            remaining,
        }
    }
}
#[derive(Clone, Debug)]
pub struct MonthPrayerTimes {
    pub days: Vec<DayPrayerTimes>,
    pub date: NaiveDate,
}
#[derive(Clone, Debug)]
pub struct CalendarPrayerTimes {
    pub months: VecDeque<MonthPrayerTimes>,
}
impl DayPrayerTimes {}

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
        let fajr = time_with_offset(
            &self.city.coordinates,
            self.city.timezone.utc_offset,
            prayers.time(Fajr),
        );
        let sunrise = time_with_offset(
            &self.city.coordinates,
            self.city.timezone.utc_offset,
            prayers.time(Sunrise),
        );
        let dhuhr = time_with_offset(
            &self.city.coordinates,
            self.city.timezone.utc_offset,
            prayers.time(Dhuhr),
        );
        let asr = time_with_offset(
            &self.city.coordinates,
            self.city.timezone.utc_offset,
            prayers.time(Asr),
        );
        let maghrib = time_with_offset(
            &self.city.coordinates,
            self.city.timezone.utc_offset,
            prayers.time(Maghrib),
        );
        let isha = time_with_offset(
            &self.city.coordinates,
            self.city.timezone.utc_offset,
            prayers.time(Isha),
        );
        let fajr_tomorrow = time_with_offset(
            &self.city.coordinates,
            self.city.timezone.utc_offset,
            prayers.time(FajrTomorrow),
        );

        Ok(DayPrayerTimes {
            date: *naive_date,
            hijri: hijri_date,
            fajr,
            sunrise,
            dhuhr,
            asr,
            maghrib,
            isha,
            fajr_tomorrow,
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
        for _ in 0..(MONTHS_PRELOAD + 1) {
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
    pub fn calculate_with_angles() {}
}

pub fn time_with_offset(
    coordinates: &Coordinates,
    offset: i64,
    date_time: DateTime<Utc>,
) -> DateTime<Tz> {
    // 1. Try looking up and parsing the IANA timezone
    let calculated_time = tz_search::lookup(coordinates.latitude, coordinates.longitude)
        .and_then(|tz_str| tz_str.parse::<Tz>().ok())
        .map(|tz| date_time.with_timezone(&tz));

    // 2. Fallback: Parse fixed offset or apply static seconds
    calculated_time.unwrap_or_else(|| {
        // Apply the static seconds offset to UTC
        let shifted_utc = date_time
            .checked_add_signed(TimeDelta::seconds(offset))
            .unwrap_or(date_time);

        // Represent the shifted time as UTC (or convert to FixedOffset)
        shifted_utc.with_timezone(&Tz::UTC)
    })
}

#[cfg(test)]
mod test {
    use crate::core::time::time_with_offset;
    use crate::core::types::{
        City, Coordinates, Country, Madhab, Method, Region, Timezone, UserData,
    };
    use chrono::Utc;
    use chrono_tz::Tz;

    #[test]
    fn check_time_with_offset_in_different_timezones() {
        let country = Country::new(
            "PS".to_string(),
            "Palestine".to_string(),
            Madhab::Hanafi,
            Method::MuslimWorldLeague,
        );
        let city = City::new(
            "tel aviv".to_string(),
            Coordinates::new(36.16525, 1.33452),
            Timezone::new(3600),
        );
        let region = Region::new("chlef".to_string());
        let data = UserData::new(country, region, city);
        let _time = time_with_offset(
            &data.city.coordinates,
            data.city.timezone.utc_offset,
            Utc::now(),
        );

        let coordinates = Coordinates::new(32.08088, 34.78057);
        let calculated_time = tz_search::lookup(coordinates.latitude, coordinates.longitude)
            .and_then(|tz_str| tz_str.parse::<Tz>().ok())
            .and_then(|tz| {
                // Interpret the wall-clock time in the local timezone (respects DST)
                Utc::now()
                    .naive_utc()
                    .and_local_timezone(tz)
                    .latest() // Handles DST transitions safely
                    .map(|local_dt| local_dt.with_timezone(&Utc))
            });
        let timzone = tz_search::lookup(coordinates.latitude, coordinates.longitude).unwrap();
        let timezone = timzone.parse::<Tz>().unwrap();
        let time = Utc::now().with_timezone(&timezone);
        println!("{:?}", calculated_time);
        println!("{:?}", time);
        println!("{:?}", data.city.name);
        println!("{:?}", time);
    }
}
