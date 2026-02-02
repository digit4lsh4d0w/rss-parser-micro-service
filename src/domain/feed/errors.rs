use thiserror::Error;

#[derive(Debug, Error)]
pub enum FeedError {
    #[error("Name cannot be empty")]
    EmptyName,
    #[error("URL cannot be empty")]
    EmptyUrl,
    #[error("URL is invalid: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Update interval must be at least {min} minutes, got: {got}")]
    InvalidUpdateInterval { min: u64, got: u64 },
    #[error(
        "Update retries must meet the following conditions: {min} <= retries <= {max}, got: {got}"
    )]
    InvalidUpdateRetries { min: usize, max: usize, got: usize },
}
