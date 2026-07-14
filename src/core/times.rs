use crate::core::types::{DayPrayerEntries, ErrorType, Location};
use chrono::{DateTime, Utc};
use salah::{Configuration, Coordinates, Madhab, Method, Prayer, PrayerSchedule};

fn get_time_now() -> DateTime<Utc> {
    Utc::now()
}
fn get_madhab(location: Location) {}
fn get_method(location: Location) {}
pub fn calculate_prayer_times(
    date_time: DateTime<Utc>,
    location: Location,
    madhab: Madhab,
    method: Method,
) -> Result<DayPrayerEntries, ErrorType> {
    let city = Coordinates::new(location.latitude, location.longitude);
    let params = Configuration::with(method, madhab);
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
    use crate::core::types::Location;
    use salah::{Madhab, Method};

    fn test_calculate_prayer_times() {
        let location = Location {
            latitude: 36.193721,
            longitude: 1.260718,
        };
        let madhab = Madhab::Shafi;
        let method = Method::MuslimWorldLeague;
        let date_time = get_time_now();
        assert!(calculate_prayer_times(date_time, location, madhab, method).is_ok());
    }
}
