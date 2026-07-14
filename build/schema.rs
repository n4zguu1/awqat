use rusqlite::Connection;
use std::error::Error;
use std::path::Path;

fn db_creation(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let connection = Connection::open(file_path)?;
    connection
        .execute_batch(
            "CREATE TABLE Countries (
                Id int PRIMARY KEY,
                Name varchar(255) NOT NULL,
                Madhab varchar(255) ,
                Method varchar(255)  ,
                FajrAngle int NOT NULL,
                IshaAngle int NOT NULL,
                );
            CREATE TABLE Cities (
                Id int PRIMARY KEY,
                CountryId int,
                Name varchar(255) NOT NULL ,
                latitude float NOT NULL,
                longitude float NOT NULL,
                FOREIGN KEY(CountryId) REFERENCES Countries(Id)
                );
    ",
        )
        .expect("failed to create the database");

    Ok(())
}
