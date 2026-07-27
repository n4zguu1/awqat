// handle all logic related to time unit

use crate::core::date::NaiveHijriDate;

use crate::core::types::{Method, UserData};
use crate::error::ErrorType;
use chrono::{DateTime, Local, NaiveDate, Utc};
use salah::Prayer::{Asr, Dhuhr, Fajr, Isha, Maghrib, Sunrise};
use salah::{Configuration, PrayerSchedule, TimeAdjustment};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub struct BatchPrayers {
    base: NaiveDate,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct PrayerTimes {
    pub fajr: DateTime<Utc>,
    pub sunrise: DateTime<Utc>,
    pub dhuhr: DateTime<Utc>,
    pub asr: DateTime<Utc>,
    pub maghrib: DateTime<Utc>,
    pub isha: DateTime<Utc>,
}

impl PrayerTimes {
    fn new(
        fajr: DateTime<Utc>,
        sunrise: DateTime<Utc>,
        dhuhr: DateTime<Utc>,
        asr: DateTime<Utc>,
        maghreb: DateTime<Utc>,
        isha: DateTime<Utc>,
    ) -> Self {
        Self {
            fajr,
            sunrise,
            dhuhr,
            asr,
            maghrib: maghreb,
            isha,
        }
    }
    pub fn as_string_with_timezone(&self) -> [String; 6] {
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

        [fajr, sunrise, dhuhr, asr, maghrib, isha]
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
    pub fn calculate(&self, naive_date: &NaiveDate) -> Result<PrayerTimes, ErrorType> {
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

        Ok(PrayerTimes::new(fajr, sunrise, dhuhr, asr, maghrib, isha))
    }
    #[allow(dead_code)]
    pub fn calculate_batch(&self) {}
    #[allow(dead_code)]
    pub fn calculate_with_angles() {}
}
