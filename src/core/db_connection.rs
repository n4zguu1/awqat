use crate::core::embeddings::embed;
use crate::core::error::ErrorType;
use crate::core::types::APP_VERSION;
use rusqlite::Connection;

pub fn connection() -> Result<Connection, ErrorType> {
    embed()?;
    let file_name = format!("awqat_{}.db", APP_VERSION);
    let mut temp_file_path = std::env::temp_dir();
    temp_file_path.push("awqat");
    temp_file_path.push(file_name);
    println!("{:?}", temp_file_path.to_str());
    let conn = Connection::open(&temp_file_path).map_err(ErrorType::SqliteConnectionOpenFailed)?;
    Ok(conn)
}
