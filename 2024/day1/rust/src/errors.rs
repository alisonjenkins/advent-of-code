use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to read data from: {path} due to error: {source}")]
    ReadData {
        path: std::path::PathBuf,
        source: std::io::Error,
    },

    #[error("Hit the end of the data when trying to find the data entry lengths")]
    EntryLengthAtEnd {},

    #[error("Failed to get entry 1 text")]
    FailedToGetEntry1Text,

    #[error("Failed to get entry 2 text")]
    FailedToGetEntry2Text,

    #[error("Failed to parse entry 1 text: {source}")]
    FailedToParseEntry1Text { source: std::num::ParseIntError },

    #[error("Failed to parse entry 2 text: {source}")]
    FailedToParseEntry2Text { source: std::num::ParseIntError },
}
