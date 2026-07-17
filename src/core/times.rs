use crate::core::types::{Data, DayPrayerEntries, ErrorType, MetaData};
use chrono::{DateTime, Utc};
use salah::{Configuration, Coordinates, Madhab, Method, Prayer, PrayerSchedule};

fn get_time_now() -> DateTime<Utc> {
    Utc::now()
}
pub fn calculate_prayer_times(
    date_time: DateTime<Utc>,
    params: MetaData,
) -> Result<DayPrayerEntries, ErrorType> {
    let city = Coordinates::new(params.coordinates.latitude, params.coordinates.longitude);
    let params = if params.method == Method::Other {
        Configuration::new(params.angles.fajr, params.angles.isha).done()
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
