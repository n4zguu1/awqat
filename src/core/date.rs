// handle all logic related to dates.especcially hijri date calcualtion

// the Hijri dates dynamically change on each country, depending on visual insight , algorithms , hybrid approach.
// in that manner, we use this mechanism to make the date accurate.

// approach one :
// api request on popular providers
// approach two :
// we fallback to Umm al-Qura calcualtion method
// approach three:
// make the user mannual configure the hijri date based on local data

use crate::error::ErrorType;
use chrono::{Datelike, NaiveDate};
use icu_calendar::Date;
use icu_calendar::cal::Hijri;
use std::fmt::{Display, Formatter};

pub enum HijriMonths {
    Muharram = 1,
    Safar = 2,
    RabiAlAwwal = 3,
    RabiAlThani = 4,
    JumadaAlAwwal = 5,
    JumadaAlThani = 6,
    Rajab = 7,
    Shaban = 8,
    Ramadan = 9,
    Shawwal = 10,
    DhuAlQadah = 11,
    DhuAlHijjah = 12,
}
impl HijriMonths {
    pub fn from_number(month: u8) -> Option<Self> {
        match month {
            1 => Some(HijriMonths::Muharram),
            2 => Some(HijriMonths::Safar),
            3 => Some(HijriMonths::RabiAlAwwal),
            4 => Some(HijriMonths::RabiAlThani),
            5 => Some(HijriMonths::JumadaAlAwwal),
            6 => Some(HijriMonths::JumadaAlThani),
            7 => Some(HijriMonths::Rajab),
            8 => Some(HijriMonths::Shaban),
            9 => Some(HijriMonths::Ramadan),
            10 => Some(HijriMonths::Shawwal),
            11 => Some(HijriMonths::DhuAlQadah),
            12 => Some(HijriMonths::DhuAlHijjah),
            _ => None,
        }
    }
}

impl Display for HijriMonths {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            HijriMonths::Muharram => "Muharram",
            HijriMonths::Safar => "Safar",
            HijriMonths::RabiAlAwwal => "Rabi Al-Awwal",
            HijriMonths::RabiAlThani => "Rabi Al-Thani",
            HijriMonths::JumadaAlAwwal => "Jumada Al-Awwal",
            HijriMonths::JumadaAlThani => "Jumada Al-Thani",
            HijriMonths::Rajab => "Rajab",
            HijriMonths::Shaban => "Shaban",
            HijriMonths::Ramadan => "Ramadan",
            HijriMonths::Shawwal => "Shawwal",
            HijriMonths::DhuAlQadah => "Dhu Al-Qadah",
            HijriMonths::DhuAlHijjah => "Dhu Al-Hijjah",
        };
        write!(f, "{}", str)
    }
}

pub struct NaiveHijriDate {
    pub year: i32,
    pub month_name: HijriMonths,
    pub month: u8,
    pub day: u8,
}
impl Display for NaiveHijriDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.day, self.month_name, self.year)
    }
}

// todo: the hijri date doest work on before hijrah, it delivers wrong dates.
impl NaiveHijriDate {
    // before Hijrah years are prefixed by '-'
    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, ErrorType> {
        if !(1..=12).contains(&month) {
            return Err(ErrorType::MonthParamError);
        } else if !(1..=30).contains(&day) {
            return Err(ErrorType::DayParamError);
        }
        let month_name = if let Some(hijri) = HijriMonths::from_number(month) {
            hijri
        } else {
            return Err(ErrorType::MonthParamError);
        };
        Ok(NaiveHijriDate {
            month,
            year,
            day,
            month_name,
        })
    }
    #[allow(dead_code)]
    pub fn to_numeric(&self) -> NumericHijriDate<'_> {
        NumericHijriDate(self)
    }
    pub fn from_gregorian_to_ummalqura(gregorian_date: &NaiveDate) -> Result<Self, ErrorType> {
        let calendar = Hijri::new_umm_al_qura();

        let iso_date = Date::try_new_iso(
            gregorian_date.year(),
            gregorian_date.month() as u8,
            gregorian_date.day() as u8,
        )
        .map_err(ErrorType::HijriDateInitializationFailed)?;

        let date = iso_date.to_calendar(calendar);
        let year = date.year().era_year_or_related_iso();
        let month = date.month().ordinal;
        let day = date.day_of_month().0;
        NaiveHijriDate::new(year, month, day)
    }
}
#[allow(dead_code)]
pub struct NumericHijriDate<'a>(&'a NaiveHijriDate);
impl<'a> Display for NumericHijriDate<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.0.day, self.0.month, self.0.year)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::date::NaiveHijriDate;
    use chrono::NaiveDate;

    #[test]
    pub fn ummalqura_hijri_date() {
        let hijri = NaiveHijriDate::from_gregorian_to_ummalqura(
            &NaiveDate::from_ymd_opt(2014, 2, 1).unwrap(),
        )
        .unwrap();
        let hijri_numeric = hijri.to_numeric();

        assert_eq!(format!("{}", hijri_numeric), "1-4-1435");
    }
}
