use crate::core::types::Location;
use salah::{Madhab, Method};

pub struct DataSet {
    country: String,
    madhab: Madhab,
    method: Method,
    cities: Vec<City>,
}
struct City {
    name: String,
    coordinates: Location,
}
