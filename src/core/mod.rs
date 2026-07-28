use crate::core::storage::{get_config_path, load_config};
use crate::core::types::UserData;
use chrono::{Local, TimeZone, Utc};
use std::path::Path;

pub mod date;
pub mod embeddings;
pub mod search;
pub mod storage;
pub mod time;
pub mod types;

pub fn init_core() {
    let path = Path::new("/mnt/workspace/Projects/awqat/data/trash/trash.json");
    let config = get_config_path().unwrap();
    let data: UserData = load_config(&config).unwrap();
    let mut batch = data.calculate_batch(&Local::now().date_naive()).unwrap();
    let prayers = data.calculate(&Utc::now().date_naive()).unwrap();
    let next = prayers.next_prayer(&Utc.with_ymd_and_hms(2026, 7, 28, 18, 13, 0).unwrap());

    println!("{:?}", next);
    println!("{:?}", prayers);
    // data.scroll_up(&mut batch).unwrap();
    // save_config(&batch, path).unwrap();
}
