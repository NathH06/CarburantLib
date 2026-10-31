use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("invalid JSON data: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("Failed to search data : {0}")]
    DataError(#[from] DataError),
}