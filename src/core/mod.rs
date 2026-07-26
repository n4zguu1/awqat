use crate::core::storage::{db_connection, save_config};
use crate::core::types::UserData;
use chrono::Utc;
use std::path::Path;

pub mod date;
pub mod embeddings;
pub mod search;
pub mod storage;
pub mod time;
pub mod types;

pub fn init_core() {
    let conn = db_connection().unwrap();
    let data = UserData::from_city_id(442512, &conn).unwrap();
    let prayer_time = data.calculate(&Utc::now().date_naive()).unwrap();

    let path = Path::new("/mnt/workspace/Projects/awqat/data/trash/data.json");
    save_config(&prayer_time, path).unwrap();
}
