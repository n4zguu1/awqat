use crate::core::embeddings::embed;
use crate::core::types::{APP_VERSION, UserData};
use crate::error::ErrorType;
use directories::ProjectDirs;
use rusqlite::Connection;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};

pub fn db_connection() -> Result<Connection, ErrorType> {
    embed()?;
    let file_name = format!("awqat_{}.db", APP_VERSION);
    let mut temp_file_path = std::env::temp_dir();
    temp_file_path.push("awqat");
    temp_file_path.push(file_name);
    println!("{:?}", temp_file_path.to_str());
    let conn = Connection::open(&temp_file_path).map_err(ErrorType::SqliteConnectionOpenFailed)?;
    Ok(conn)
}
pub fn save_config<T>(data: &T) -> Result<(), ErrorType>
where
    T: Serialize,
{
    let config_path = if let Some(proj_dirs) = ProjectDirs::from("com", "awqat", "awqat") {
        proj_dirs.config_dir().to_path_buf()
    } else {
        return Err(ErrorType::ConfigDirError);
    };
    std::fs::create_dir_all(&config_path)?;
    let file_name = format!("config_{}.json", APP_VERSION);
    let mut file_path = config_path.clone();
    file_path.push(file_name);
    let file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open(&file_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &data).map_err(ErrorType::SerializeError)?;

    Ok(())
}
pub fn load_config<T>() -> Result<T, ErrorType>
where
    T: DeserializeOwned,
{
    let config_path = if let Some(proj_dirs) = ProjectDirs::from("com", "awqat", "awqat") {
        proj_dirs.config_dir().to_path_buf()
    } else {
        return Err(ErrorType::ConfigDirError);
    };
    let file_name = format!("config_{}.json", APP_VERSION);
    let mut config_file_path = config_path.clone();
    config_file_path.push(file_name);
    let file = OpenOptions::new().read(true).open(&config_file_path)?;
    let reader = BufReader::new(file);
    let data: T = serde_json::from_reader(reader).map_err(ErrorType::DeserializeError)?;
    Ok(data)
}
