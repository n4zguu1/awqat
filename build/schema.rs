use crate::parser::parse::parse_dataset;
use rusqlite::{Connection, Result, params};
use std::error::Error;
use std::path::Path;

pub fn db_creation(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut connection = Connection::open(file_path)?;

    connection.execute_batch(
        "CREATE TABLE IF NOT EXISTS Countries (
            Id INTEGER PRIMARY KEY AUTOINCREMENT,
            Iso2 TEXT NOT NULL UNIQUE,
            Name TEXT NOT NULL,
            Native TEXT,
            Method TEXT NOT NULL,
            FajrAngle REAL,
            IshaAngle REAL
        );
        CREATE TABLE IF NOT EXISTS Regions (
            Id INTEGER PRIMARY KEY AUTOINCREMENT,
            CountryId INTEGER NOT NULL,
            Name TEXT NOT NULL,
            FOREIGN KEY(CountryId) REFERENCES Countries(Id)
        );
        CREATE TABLE IF NOT EXISTS Cities (
            Id INTEGER PRIMARY KEY AUTOINCREMENT,
            RegionId INTEGER NOT NULL,
            Name TEXT NOT NULL,
            Latitude REAL NOT NULL,
            Longitude REAL NOT NULL,
            TimezoneName TEXT NOT NULL,
            GmtOffset INTEGER NOT NULL,
            FOREIGN KEY(RegionId) REFERENCES Regions(Id)
        );",
    )?;

    let data = parse_dataset();

    let tx = connection.transaction()?;
    tx.execute_batch("DELETE FROM Cities; DELETE FROM Regions; DELETE FROM Countries;")?;
    {
        let mut country_stmt = tx.prepare(
            "INSERT INTO Countries (Iso2, Name, Native, Method, FajrAngle, IshaAngle)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;

        let mut region_stmt = tx.prepare(
            "INSERT INTO Regions (CountryId, Name)
             VALUES (?1, ?2)",
        )?;

        let mut city_stmt = tx.prepare(
            "INSERT INTO Cities (RegionId, Name, Latitude, Longitude, TimezoneName, GmtOffset)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;

        // Cache to avoid duplicate regions
        let mut region_cache: std::collections::HashMap<(String, String), i64> = std::collections::HashMap::new();

        for country in &data.countries {
            // Insert country
            let country_id = country_stmt.insert(params![
                country.iso2,
                country.name,
                country.native,
                country.method,
                country.angles.fajr,
                country.angles.isha,
            ])?;

            for region in &country.region {
                // Get or insert region
                let region_key = (country.iso2.clone(), region.name.clone());
                let region_id = if let Some(&id) = region_cache.get(&region_key) {
                    id
                } else {
                    let id = region_stmt.insert(params![
                        country_id,
                        region.name,
                    ])?;
                    region_cache.insert(region_key, id);
                    id
                };

                // Insert cities
                for city in &region.city {
                    city_stmt.execute(params![
                        region_id,
                        city.name,
                        city.coordinates.latitude,
                        city.coordinates.longitude,
                        city.timezone.tz_name,
                        city.timezone.gmt_offset,
                    ])?;
                }
            }
        }
    }

    tx.commit()?;

    Ok(())
}
