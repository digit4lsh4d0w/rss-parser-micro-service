use std::path::Path;

use serde::Deserialize;

use feed::FeedConfig;

use crate::config::{
    error::{ConfigError, ConfigResult, ValidationError, ValidationResult},
    feed::{DEFAULT_UPDATE_INTERVAL, DEFAULT_UPDATE_RETRIES, RawFeedConfig},
    validation::{validate_update_interval, validate_update_retries},
};

pub mod error;
mod feed;
mod notification;
mod validation;

/// Основные настройки приложения
#[derive(Debug)]
pub struct Config {
    /// Список фидов
    pub feeds: Vec<FeedConfig>,
    // pub notification_service: NotificationServiceConfig,
}

impl TryFrom<RawConfig> for Config {
    type Error = ValidationError;

    fn try_from(raw_config: RawConfig) -> Result<Self, Self::Error> {
        raw_config.validate()?;

        let feeds = raw_config
            .feeds
            .into_iter()
            .filter(|raw_feed| raw_feed.active.unwrap_or(true))
            .map(|raw_feed| FeedConfig {
                name: raw_feed.name,
                url: raw_feed.url,
                update_interval: raw_feed
                    .update_interval
                    .or(raw_config.update_interval)
                    .unwrap_or(DEFAULT_UPDATE_INTERVAL),
                update_retries: raw_feed
                    .update_retries
                    .or(raw_config.update_retries)
                    .unwrap_or(DEFAULT_UPDATE_RETRIES),
            })
            .collect();

        Ok(Self { feeds })
    }
}

#[derive(Deserialize)]
struct RawConfig {
    /// Список фидов
    pub feeds: Vec<RawFeedConfig>,

    /// Интервал обновлениия в секундах.
    ///
    /// Значение по умолчанию: [`DEFAULT_UPDATE_INTERVAL`].
    pub update_interval: Option<usize>,

    /// Максимальное количество попыток.
    ///
    /// Значение по умолчанию: [`DEFAULT_UPDATE_RETRIES`].
    pub update_retries: Option<usize>,
    // pub notification_service: NotificationServiceConfig,
}

impl RawConfig {
    fn validate(&self) -> ValidationResult<()> {
        self.validate_feeds()?;
        self.validate_update_interval()?;
        self.validate_update_retries()?;

        Ok(())
    }

    fn validate_feeds(&self) -> ValidationResult<()> {
        let feeds: Vec<&RawFeedConfig> = self
            .feeds
            .iter()
            .filter(|raw_feed| raw_feed.active.unwrap_or(true))
            .collect();

        if feeds.is_empty() {
            return Err(ValidationError::NoActiveFeeds);
        }

        feeds.iter().try_for_each(|raw_feed| raw_feed.validate())?;

        Ok(())
    }

    fn validate_update_interval(&self) -> ValidationResult<()> {
        if let Some(value) = self.update_interval {
            validate_update_interval(value)?;
        }

        Ok(())
    }

    fn validate_update_retries(&self) -> ValidationResult<()> {
        if let Some(value) = self.update_retries {
            validate_update_retries(value)?;
        }

        Ok(())
    }
}

pub async fn load_config(path: impl AsRef<Path>) -> ConfigResult<Config> {
    let content = tokio::fs::read_to_string(path).await?;
    let raw_config: RawConfig = toml::from_str(&content)?;
    let config = raw_config
        .try_into()
        .map_err(ConfigError::ValidationError)?;

    Ok(config)
}
