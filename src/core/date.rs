// handle all logic related to dates.especcially hijri date calcualtion

// the Hijri dates dynamically change on each country, depending on visual insight , algorithms , hybrid approach.
// in that manner, we use this mechanism to make the date accurate.

// approach one :
// api request on popular providers
// approach two :
// we fallback to Umm al-Qura calcualtion method
// approach three:
// make the user mannual configure the hijri date based on local data

use crate::core::error::ErrorType;
use chrono::{Datelike, NaiveDate};
use icu_calendar::Date;
use icu_calendar::cal::Hijri;
use icu_calendar::cal::hijri::UmmAlQura;
use std::fmt::{Display, Formatter, write};

pub enum HijriMonths {
    Muharram,
    Safar,
    RabiAlAwwal,
    RabiAlThani,
    JumadaAlAwwal,
    JumadaAlThani,
    Rajab,
    Shaban,
    Ramadan,
    Shawwal,
    DhuAlQadah,
    DhuAlHijjah,
}
impl HijriMonths {
    pub fn from_number(month: u8) -> Self {
        match month {
            1 => HijriMonths::Muharram,
            2 => HijriMonths::Safar,
            3 => HijriMonths::RabiAlAwwal,
            4 => HijriMonths::RabiAlThani,
            5 => HijriMonths::JumadaAlAwwal,
            6 => HijriMonths::JumadaAlThani,
            7 => HijriMonths::Rajab,
            8 => HijriMonths::Shaban,
            9 => HijriMonths::Ramadan,
            10 => HijriMonths::Shawwal,
            11 => HijriMonths::DhuAlQadah,
            12 => HijriMonths::DhuAlHijjah,
            _ => unreachable!("a year can have only 12 months"),
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

pub struct HijriDate {
    pub year: i32,
    pub month_name: HijriMonths,
    pub month: u8,
    pub day: u8,
}
impl Display for HijriDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.day, self.month_name, self.year)
    }
}

// todo: the hijri date doesn work on before hijrah, it delivers wrong dates.
impl HijriDate {
    // before Hijrah years are prefixed by '-'
    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, ErrorType> {
        if month > 12 || month < 1 {
            return Err(ErrorType::MonthParamError);
        } else if day > 30 || day < 1 {
            return Err(ErrorType::DayParamError);
        }
        let month_name = HijriMonths::from_number(month);
        Ok(HijriDate {
            month,
            year,
            day,
            month_name,
        })
    }
    pub fn to_numeric(&self) -> NumericHijriDate {
        NumericHijriDate(self)
    }
    pub fn from_gregorian_to_ummalqura(gregorian_date: NaiveDate) -> Result<Self, ErrorType> {
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
        let month_name = HijriMonths::from_number(month);
        Ok(HijriDate {
            year,
            month,
            day,
            month_name,
        })
    }
}
pub struct NumericHijriDate<'a>(&'a HijriDate);
impl<'a> Display for NumericHijriDate<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.0.day, self.0.month, self.0.year)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::date::HijriDate;
    use chrono::NaiveDate;

    #[test]
    pub fn ummalqura_hijri_date() {
        let hijri =
            HijriDate::from_gregorian_to_ummalqura(NaiveDate::from_ymd_opt(700, 2, 1).unwrap())
                .unwrap();
        // Extract numerical representations
        println!("{}", hijri)
        // assert_eq!(hijri_str, "1/4/1435");
    }
}
