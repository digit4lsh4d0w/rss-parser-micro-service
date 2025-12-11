use anyhow::Result;
use thiserror::Error;

use crate::config::{
    feed::{MAX_UPDATE_RETRIES, MIN_UPDATE_INTERVAL, MIN_UPDATE_RETRIES},
    notification::MIN_NOTIFICATION_TIMEOUT,
};

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
    #[error("URL cannot be empty")]
    EmptyUrl,
    #[error("Invalid URL format")]
    InvalidUrlFormat,
    #[error("Feed name cannot be empty")]
    FeedEmptyName,
    #[error("Update interval must be at least {MIN_UPDATE_INTERVAL} seconds, got: {0}")]
    FeedUpdateIntervalTooSmall(usize),
    #[error("Update retries must be at least {MIN_UPDATE_RETRIES}, got: {0}")]
    FeedUpdateRetriesTooSmall(usize),
    #[error("Update retries must be no more than {MAX_UPDATE_RETRIES}, got: {0}")]
    FeedUpdateRetriesTooBig(usize),
    #[error("Notification service timeout must be at least {MIN_NOTIFICATION_TIMEOUT}, got: {0}")]
    NotificationServiceTimeoutTooSmall(usize),
    #[error("No active feeds found in configuration")]
    NoActiveFeeds,
}

pub type ConfigResult<T> = Result<T, ConfigError>;
pub type ValidationResult<T> = Result<T, ValidationError>;
