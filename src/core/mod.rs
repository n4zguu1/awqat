use crate::core::search::selected_city;
use crate::core::storage::{db_connection, load_config, save_config};

pub mod date;
pub mod embeddings;
pub mod search;
pub mod storage;
pub mod time;
pub mod types;

pub fn init_core() {
    let conn = db_connection().unwrap();
    let data = selected_city(442512, &conn).unwrap();
    save_config(&data).unwrap();
}
