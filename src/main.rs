use crate::core::db_connection::connection;
use crate::core::error::ErrorType;

mod core;
mod ui;

fn main() -> Result<(), ErrorType> {
    connection()?;
    Ok(())
}
