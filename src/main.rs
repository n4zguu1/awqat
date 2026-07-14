mod core;
mod parser;

use salah::prelude::*;

fn main() {
    let chlef = Coordinates::new(36.198464 ,1.250709);
    let date = NaiveDate::from_ymd_opt(2026, 07, 14).expect("Invalid date provided");
    let params = Configuration::with(Method::MuslimWorldLeague, Madhab::Shafi);
    let prayers = PrayerSchedule::new()
        .on(date)
        .for_location(chlef)
        .with_configuration(params)
        .calculate();

    println!("{:?}",prayers);
}
