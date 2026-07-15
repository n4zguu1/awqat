use crate::parser::parse::parse_dataset;
use rusqlite::{Connection, Result, params};
use std::error::Error;
use std::path::Path;

pub fn db_creation(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut connection = Connection::open(file_path)?;

    connection.execute_batch(
        "CREATE TABLE IF NOT EXISTS Countries (
            Id INTEGER PRIMARY KEY AUTOINCREMENT,
            Name TEXT NOT NULL,
            Madhab TEXT,
            Method TEXT,
            FajrAngle INTEGER NOT NULL,
            IshaAngle INTEGER NOT NULL
        );
        CREATE TABLE IF NOT EXISTS Cities (
            Id INTEGER PRIMARY KEY AUTOINCREMENT,
            CountryId INTEGER,
            Name TEXT NOT NULL,
            latitude REAL NOT NULL,
            longitude REAL NOT NULL,
            FOREIGN KEY(CountryId) REFERENCES Countries(Id)
        );",
    )?;

    let data = parse_dataset();

    let tx = connection.transaction()?;
    tx.execute_batch("DELETE FROM Cities; DELETE FROM Countries;")?;
    {
        let mut country_stmt = tx.prepare(
            "INSERT INTO Countries (Name, Madhab, Method, FajrAngle, IshaAngle)
             VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;

        let mut city_stmt = tx.prepare(
            "INSERT INTO Cities (CountryId, Name, latitude, longitude)
             VALUES (?1, ?2, ?3, ?4)",
        )?;

        for entry in data.countries {
            let country_id = country_stmt.insert(params![
                entry.country.name,
                entry.madhab,
                entry.method,
                entry.fajr_angle,
                entry.isha_angle,
            ])?;

            for city in entry.country.cities {
                city_stmt.execute(params![
                    country_id,
                    city.name,
                    city.coordinates.latitude,
                    city.coordinates.longitude,
                ])?;
            }
        }
    }

    tx.commit()?;

    Ok(())
}
