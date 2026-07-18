// we can use automated way to fetch user ip location , but that isnt accurate, and completely based on ISP provider.
// also, it still requires user to input the city, which we tried to prevent by implementing automated way
// this options remains un functional till we discover another way arround

// the only options remains is make user mannually imput the location
// there is some issue to address
// the duplicates, what if there is two cities named with same name

use crate::core::error::ErrorType;
use crate::core::types::Data;
use rusqlite::{Connection, params};
use std::collections::HashMap;
// we use FTS5 , found out that rusqlite supports it out of the box
// DEFINITION:
// creates an inverted index search tables inside the db binary, for each token on the db, maps them to where they appear. at the end we have a huge lookup table
// the overhead it cuzes is it doubles down the binary size
// slower writes and updates, cuz each update the table needs to update too

pub fn search_city(conn: Connection, name: &str) -> Result<HashMap<i64, String>, ErrorType> {
    let query = "select rowid,name from cities_fts where name match ?1";
    let mut statement = conn
        .prepare(query)
        .map_err(ErrorType::SqliteOperationFailed)?;
    let results = statement
        .query_map(params![format!("{}*", name)], |row| {
            let id = row.get::<usize, i64>(0);
            let name = row.get::<usize, String>(1);
            Ok((id, name))
        })
        .map_err(ErrorType::SqliteOperationFailed)?;
    let mut hashmap = HashMap::new();
    for r in results {
        let row = r.map_err(ErrorType::SqliteOperationFailed)?;
        let id = row.0.map_err(ErrorType::SqliteOperationFailed)?;
        let name = row.1.map_err(ErrorType::SqliteOperationFailed)?;
        if hashmap.insert(id, name).is_some() {
            return Err(ErrorType::HashmapDuplicateError);
        }
    }

    Ok(hashmap)
}
pub fn select_city(id: i64) -> Result<Data, ErrorType> {

    Ok(todo!())
}
