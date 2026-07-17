use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorType {
    #[error("The requested date was not found")]
    DateNotFound,
    #[error("Error calculating the prayer time")]
    PrayerCalculationFailed,
    #[error("City was not found")]
    CityNotFound,
    #[error("Creating temp directory failed")]
    CreateTmpDirFailed(std::io::Error),
    #[error("Creating temp file failed")]
    CreateTmpFileFailed(std::io::Error),
    #[error("Writing to temp file failed")]
    WriteTmpFileFailed(std::io::Error),
    #[error("Flushing the writer failed")]
    FlushFailed(std::io::Error),
    #[error("Opening the DB connection failed")]
    ConnectionOpenFailed(rusqlite::Error),
    #[error("Rusqlite Operation failed")]
    SqliteOperationFailed(rusqlite::Error),
}