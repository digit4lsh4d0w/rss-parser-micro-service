use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileReadError(#[from] std::io::Error),
    #[error("Failed to parse TOML: {0}")]
    TomlParseError(#[from] toml::de::Error),
    #[error("Invalid feed configuration: {0}")]
    InvalidFeedConfig(String),
    #[error("No active feeds found in configuration")]
    NoActiveFeedsError,
}

#[derive(Debug, PartialEq, Error)]
pub enum FeedValidationError {
    #[error("Feed URL cannot be empty")]
    EmptyUrl,
    #[error("Invalid feed URL format")]
    InvalidUrlFormat,
    #[error("Update interval must be at least {0} seconds, got: {1}")]
    UpdateIntervalTooSmall(usize, usize),
    #[error("Update retries must be at least {0}, got: {1}")]
    UpdateRetriesTooSmall(usize, usize),
    #[error("Update retries must be no more than {0}, got: {1}")]
    UpdateRetriesTooBig(usize, usize),
}

pub type ConfigResult<T> = Result<T, ConfigError>;
pub type FeedValidationResult<T> = Result<T, FeedValidationError>;
