// we can use automated way to fetch user ip location , but that isnt accurate, and completely based on ISP provider.
// also, it still requires user to input the city, which we tried to prevent by implementing automated way
// this options remains un functional till we discover another way arround

// the only options remains is make user mannually imput the location
// there is some issue to address
// the duplicates, what if there is two cities named with same name

use crate::core::types::{City, Coordinates, Country, Madhab, Method, Region, Timezone, UserData};
use crate::error::ErrorType;
use rusqlite::{Connection, params};
use std::collections::HashMap;
// we use FTS5 , found out that rusqlite supports it out of the box
// DEFINITION:
// creates an inverted index search tables inside the db binary, for each token on the db, maps them to where they appear. at the end we have a huge lookup table
// the overhead it cuzes is it doubles down the binary size
// slower writes and updates, cuz each update the table needs to update too

#[allow(dead_code)]
pub fn search_city(conn: &Connection, name: &str) -> Result<HashMap<i64, String>, ErrorType> {
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
impl UserData {
    pub fn from_city_id(id: i64, conn: &Connection) -> Result<Self, ErrorType> {
        let sql = "
        SELECT
            ci.Name, ci.Latitude, ci.Longitude, ci.GmtOffset,
            r.Name,
            co.Iso2, co.Name, co.Method, co.Madhab
        FROM Cities ci
        JOIN Regions r    ON ci.RegionId  = r.Id
        JOIN Countries co ON r.CountryId  = co.Id
        WHERE ci.Id = ?1
    ";

        let (city_name, lat, lon, gmt, region_name, iso2, country_name, method_str, madhab_str) =
            conn.query_row(sql, params![id], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, f64>(1)?,
                    row.get::<_, f64>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, String>(6)?,
                    row.get::<_, String>(7)?,
                    row.get::<_, String>(8)?,
                ))
            })
            .map_err(ErrorType::SqliteOperationFailed)?;

        let madhab = match madhab_str.as_str() {
            "Hanafi" => Madhab::Hanafi,
            "Shafi" => Madhab::Shafi,
            other => return Err(ErrorType::UnknownMadhab(other.to_string())),
        };

        let method = match method_str.as_str() {
            "MuslimWorldLeague" => Method::MuslimWorldLeague,
            "Egyptian" => Method::Egyptian,
            "Karachi" => Method::Karachi,
            "UmmAlQura" => Method::UmmAlQura,
            "Dubai" => Method::Dubai,
            "MoonsightingCommittee" => Method::MoonsightingCommittee,
            "NorthAmerica" => Method::NorthAmerica,
            "Kuwait" => Method::Kuwait,
            "Qatar" => Method::Qatar,
            "Singapore" => Method::Singapore,
            "Tehran" => Method::Tehran,
            "Turkey" => Method::Turkey,
            other => return Err(ErrorType::UnknownMethod(other.to_string())),
        };

        let city = City::new(city_name, Coordinates::new(lat, lon), Timezone::new(gmt));
        let region = Region::new(region_name);
        let country = Country::new(iso2, country_name, madhab, method);

        Ok(UserData::new(country, region, city))
    }
}
