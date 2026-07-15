use crate::core::db_connection::connection;
use crate::core::embeddings::embed;
use crate::core::types::ErrorType;

mod core;
mod ui;

fn main() -> Result<(), ErrorType> {
    connection()?;
    Ok(())
}
