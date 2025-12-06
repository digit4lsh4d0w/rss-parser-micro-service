use anyhow::Result;
use thiserror::Error;

use crate::config::feed::{MAX_UPDATE_RETRIES, MIN_UPDATE_INTERVAL, MIN_UPDATE_RETRIES};

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileReadError(#[from] std::io::Error),
    #[error("Failed to parse TOML: {0}")]
    TomlParseError(#[from] toml::de::Error),
    #[error("Validation error: {0}")]
    ValidationError(ValidationError),
}

#[derive(Debug, PartialEq, Error)]
pub enum ValidationError {
    #[error("No active feeds found in configuration")]
    NoActiveFeeds,
    #[error("Feed name cannot be empty")]
    EmptyName,
    #[error("Feed URL cannot be empty")]
    EmptyUrl,
    #[error("Invalid feed URL format")]
    InvalidUrlFormat,
    #[error("Update interval must be at least {MIN_UPDATE_INTERVAL} seconds, got: {0}")]
    UpdateIntervalTooSmall(usize),
    #[error("Update retries must be at least {MIN_UPDATE_RETRIES}, got: {0}")]
    UpdateRetriesTooSmall(usize),
    #[error("Update retries must be no more than {MAX_UPDATE_RETRIES}, got: {0}")]
    UpdateRetriesTooBig(usize),
}

pub type ConfigResult<T> = Result<T, ConfigError>;
pub type ValidationResult<T> = Result<T, ValidationError>;
