use crate::parser::types::RawCountries;
use serde::de::Error;
use std::io;

// parse datasets into their types (e,g,. parse madhab and methods for each country)
pub fn parse_dataset() -> Result<RawCountries, Box<dyn Error>> {
    let path = "asset.json";

    Ok(todo!())
}
