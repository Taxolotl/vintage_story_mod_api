//! API error type for the VintageStory API client.

use std::io::Error;
use thiserror::Error;

/// Represents all possible errors that can occur when using the VintageStory API client.
#[derive(Error, Debug)]
pub enum ApiError {
    /// An HTTP-level error (network failure, timeout, deserialization failure, etc)
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// Any unexpected non-HTTP error
    #[error("Unexpected API error: {0}")]
    Unexpected(String),
}


impl From<std::io::Error> for ApiError {
    fn from(value: Error) -> Self {
        Self::Unexpected(value.to_string())
    }
}