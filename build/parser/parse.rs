use crate::parser::types::RawCountries;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::BufReader;
// parse datasets into their types (e,g,. parse madhab and methods for each country)
pub fn parse_dataset() -> RawCountries {
    let path = "data/dataset.json";
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("error opening the dataset file");
    let reader = BufReader::new(file);
    let data: RawCountries = serde_json::from_reader(reader).expect("error parsing the dataset");

    data
}
