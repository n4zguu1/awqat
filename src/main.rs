use crate::core::error::ErrorType;
use crate::core::search::{search_city};
use crate::core::storage::db_connection;

mod core;
mod ui;

fn main() -> Result<(), ErrorType> {
    let conn = db_connection()?;
    let results = search_city(conn, "chicago")?;
    println!("{:?}", results);
    Ok(())
}
