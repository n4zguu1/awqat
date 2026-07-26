use icu_calendar::RangeError;
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
    SqliteConnectionOpenFailed(rusqlite::Error),
    #[error("Rusqlite Operation failed")]
    SqliteOperationFailed(rusqlite::Error),
    #[error("Failed to initialize Hijri Date instance.")]
    HijriDateInitializationFailed(RangeError),
    #[error("Calculating prayer times failed.")]
    CalculatingPrayerTimesFailed(String),
    #[error("Days can only in range 1-30")]
    DayParamError,
    #[error("Months can only in range 1-12")]
    MonthParamError,
    #[error("Duplicate Key inserted to hashmap")]
    HashmapDuplicateError,
    #[error("Unknown madhab")]
    UnknownMadhab(String),
    #[error("Unknown method")]
    UnknownMethod(String),
    #[error("Installing color eyer failed")]
    ColorEyreOperationFailed(color_eyre::Report),
    #[error("io operation failed")]
    IOError(#[from] std::io::Error),
    #[error("serde serialize operation failed")]
    SerializeError(serde_json::Error),
    #[error("serde deserialize operation failed")]
    DeserializeError(serde_json::Error),
    #[error("failed to get the config directory")]
    ConfigDirError,
    #[error("config file not found")]
    ConfigFileNotFound,
}
