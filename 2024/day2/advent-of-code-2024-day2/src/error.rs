use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to read file: {path} due to error: {source}")]
    FailedToReadFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to parse input: {num} as a number. Error was: {source}")]
    FailedToParseNumber {
        num: String,
        source: std::num::ParseIntError,
    },
}
