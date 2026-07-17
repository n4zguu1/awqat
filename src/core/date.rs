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
pub struct HijriDate {
    year: u32,
    month: HijriMonths,
    ordinal: u8,
    day:u8

}
impl HijriDate {

}
pub fn hijri_date_oumalqura(
    gregorian_date: NaiveDate,
) -> Result<Date<Hijri<UmmAlQura>>, ErrorType> {
    let calendar = Hijri::new_umm_al_qura();

    let iso_date = Date::try_new_iso(
        gregorian_date.year(),
        gregorian_date.month() as u8,
        gregorian_date.day() as u8,
    )
    .map_err(ErrorType::HijriDateInitializationFailed)?;

    let date = iso_date.to_calendar(calendar);

    Ok(date)
}
#[cfg(test)]
mod tests {
    use crate::core::date::hijri_date_oumalqura;
    use chrono::NaiveDate;

    #[test]
    pub fn ummalqura_hijri_date() {
        let hijri = hijri_date_oumalqura(NaiveDate::from_ymd_opt(2014, 2, 1).unwrap()).unwrap();
        // Extract numerical representations
        let year = hijri.year().era_year_or_related_iso(); // e.g., 1447
        let month = hijri.month().ordinal; // e.g., 1 (Muharram)
        let day = hijri.day_of_month().0; // e.g., 6
        let hijri_str = format!("{}/{}/{}", day, month, year);
        assert_eq!(hijri_str, "1/4/1435");
    }
}
