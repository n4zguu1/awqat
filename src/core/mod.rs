use crate::core::storage::{get_config_path, load_config, save_config};
use crate::core::types::UserData;
use chrono::Local;
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
    let batch = data.calculate_batch(&Local::now().date_naive()).unwrap();
    save_config(&batch, path).unwrap();
}
