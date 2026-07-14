use crate::core::types::Location;
use rusqlite::Connection;

// from a table index , we choose
fn manual_lookup(conn: &Connection, city: &str) -> Result<Location, ()> {
    Ok(todo!())
}
