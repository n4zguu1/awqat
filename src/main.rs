use crate::core::db_connection::connection;
use crate::core::types::ErrorType;

mod core;
mod ui;
mod parser;

fn main() -> Result<(), ErrorType> {
    connection()?;
    Ok(())
}
