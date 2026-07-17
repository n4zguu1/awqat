use crate::core::types::{DayPrayerEntries, ErrorType, MetaData};
use chrono::{DateTime, Utc};
use salah::{Configuration, Coordinates, Method, Prayer, PrayerSchedule};

fn get_time_now() -> DateTime<Utc> {
    Utc::now()
}
pub fn calculate_prayer_times(
    date_time: DateTime<Utc>,
    params: MetaData,
) -> Result<DayPrayerEntries, ErrorType> {
    let city = Coordinates::new(params.coordinates.latitude, params.coordinates.longitude);
    let params = if params.method == Method::Other {
        let fajr = params.angles.fajr.unwrap_or(0.0);
        let isha = params.angles.isha.unwrap_or(0.0);
        Configuration::new(fajr, isha).done()
    } else {
        Configuration::with(params.method, params.madhab)
    };
    let date = date_time.date_naive();
    let calculate = PrayerSchedule::new()
        .on(date)
        .for_location(city)
        .with_configuration(params)
        .calculate();
    let prayer = if let Ok(prayer) = calculate {
        prayer
    } else {
        return Err(ErrorType::PrayerCalculationFailed);
    };
    let fajr = prayer.time(Prayer::Fajr);
    let sunrise = prayer.time(Prayer::Sunrise);
    let dhuhr = prayer.time(Prayer::Dhuhr);
    let asr = prayer.time(Prayer::Asr);
    let maghrib = prayer.time(Prayer::Maghrib);
    let isha = prayer.time(Prayer::Isha);
    Ok(DayPrayerEntries {
        fajr,
        sunrise,
        dhuhr,
        asr,
        maghrib,
        isha,
    })
}

#[cfg(test)]
mod test {
    use crate::core::times::{calculate_prayer_times, get_time_now};
    use salah::{Madhab, Method};

    fn test_calculate_prayer_times() {}
}
