use crate::core::storage::{db_connection, get_config_path, save_config};
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
    let prayer_time = data.calculate(&Utc::now().date_naive()).unwrap();
    println!("{:?}", prayer_time.as_string_with_timezone());
    // let path = Path::new("/mnt/workspace/Projects/awqat/data/trash/data.json");
    let path = get_config_path().unwrap();
    save_config(&data, &path).unwrap();
}
