use std::path::Path;

use serde::Deserialize;

use feed::FeedConfig;

use crate::config::{
    error::{ConfigError, ConfigResult, ValidationError, ValidationResult},
    feed::RawFeedConfig,
    notification::{NotificationServiceConfig, RawNotificationServiceConfig},
    validation::{validate_feed_update_interval, validate_feed_update_retries},
};

pub mod error;
mod feed;
mod notification;
mod validation;

/// Основные настройки приложения
#[derive(Debug)]
pub struct Config {
    /// Список конфигураций фидов
    pub feeds: Vec<FeedConfig>,
    /// Конфигурация сервиса уведомлений
    pub notification_service: NotificationServiceConfig,
}

impl TryFrom<RawConfig> for Config {
    type Error = ValidationError;

    fn try_from(raw_config: RawConfig) -> Result<Self, Self::Error> {
        raw_config.validate()?;

        let active_raw_feeds: Vec<RawFeedConfig> = raw_config
            .feeds
            .into_iter()
            .filter(|raw_feed| raw_feed.active.unwrap_or(true))
            .collect();

        if active_raw_feeds.is_empty() {
            return Err(ValidationError::NoActiveFeeds);
        }

        let feeds = active_raw_feeds
            .into_iter()
            .map(|raw_feed| {
                FeedConfig::try_from_raw_feed_config(
                    raw_feed,
                    raw_config.update_interval,
                    raw_config.update_retries,
                )
            })
            .collect::<ValidationResult<Vec<FeedConfig>>>()?;

        let notification_service = raw_config.notification_service.try_into()?;

        Ok(Self {
            feeds,
            notification_service,
        })
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
    pub notification_service: RawNotificationServiceConfig,
}

impl RawConfig {
    fn validate(&self) -> ValidationResult<()> {
        if let Some(value) = self.update_interval {
            validate_feed_update_interval(value)?;
        }
        if let Some(value) = self.update_retries {
            validate_feed_update_retries(value)?;
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
