use crate::core::storage::{db_connection, load_config, save_config};
use crate::core::time::PrayerData;
use crate::core::types::UserData;
use chrono::Utc;

pub mod date;
pub mod embeddings;
pub mod search;
pub mod storage;
pub mod time;
pub mod types;

pub fn init_core() {
    let conn = db_connection().unwrap();
    let data = UserData::from_city_id(442512, &conn).unwrap();
    let prayers = PrayerData::from_data(&data);
    let prayer_times = prayers.calculate(&Utc::now().date_naive()).unwrap();
    save_config(&prayers).unwrap();
}
