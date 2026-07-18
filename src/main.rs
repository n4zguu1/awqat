use crate::core::error::ErrorType;
use crate::core::search::{search_city, selected_city};
use crate::core::storage::db_connection;
use crate::core::time::PrayerData;
use chrono::Utc;

mod core;
mod ui;

fn main() -> Result<(), ErrorType> {
    let conn = db_connection()?;
    let results = *search_city(&conn, "mecca")?.keys().next().unwrap();
    let data = selected_city(results, &conn)?;
    let prayer_data = PrayerData::new(data);
    let times = prayer_data.calculate_prayer_times(&Utc::now())?;

    println!("{:?}", times);
    Ok(())
}
