use thiserror::Error;

use crate::config::feed::{MAX_UPDATE_RETRIES, MIN_UPDATE_INTERVAL, MIN_UPDATE_RETRIES};

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileReadError(#[from] std::io::Error),
    #[error("Failed to parse TOML: {0}")]
    TomlParseError(#[from] toml::de::Error),
    #[error("Feed configuration error: {0}")]
    FeedError(#[from] FeedError),
    #[error("Queue configuration error: {0}")]
    QueueError(#[from] QueueError),
    #[error("No active feeds found in configuration")]
    NoActiveFeeds,
}

#[derive(Debug, Error)]
pub enum FeedError {
    #[error("Name cannot be empty")]
    EmptyName,
    #[error("URL cannot be empty")]
    EmptyUrl,
    #[error("URL is invalid: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Update interval must be at least {MIN_UPDATE_INTERVAL} seconds, got: {0}")]
    InvalidUpdateInterval(i64),
    #[error(
        "Update retries must meet the following conditions: {MIN_UPDATE_RETRIES} <= retries <= {MAX_UPDATE_RETRIES}, got: {0}"
    )]
    InvalidUpdateRetries(usize),
}

#[derive(Debug, Error)]
pub enum QueueError {
    #[error("Endpoint cannot be empty")]
    EmptyEndpoint,
    #[error("Exchange name cannot be empty")]
    EmptyExchangeName,
    #[error("Username cannot be empty")]
    EmptyUsername,
    #[error("Password cannot be empty")]
    EmptyPassword,
}
