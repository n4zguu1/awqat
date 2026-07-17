use crate::parser::types::AwqatData;
use std::fs::OpenOptions;
use std::io::BufReader;

pub fn parse_dataset() -> AwqatData {
    let path = "data/awqat.json";
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("error opening the awqat.json file");
    let reader = BufReader::new(file);
    let data: AwqatData = serde_json::from_reader(reader).expect("error parsing awqat.json");

    data
}
