use thiserror::Error;

use crate::domain::feed::errors::FeedError;

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
