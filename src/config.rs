use std::path::Path;

use serde::Deserialize;

use error::ConfigError;
use feed::{FeedConfig, RawFeedConfig};
use queue::QueueConfig;

pub mod error;
mod feed;
mod queue;

/// Основные настройки приложения
#[derive(Debug)]
pub struct Config {
    /// Список конфигураций фидов
    pub feeds: Vec<FeedConfig>,

    /// Конфигурация очереди
    pub queue: QueueConfig,
}

impl TryFrom<RawConfig> for Config {
    type Error = ConfigError;

    fn try_from(raw_config: RawConfig) -> Result<Self, Self::Error> {
        raw_config.validate()?;

        // Конвертация сырых активных фидов в обогащенные
        let feeds = raw_config
            .feeds
            .into_iter()
            .filter(|rf| rf.active.unwrap_or(true))
            .map(|rf| {
                FeedConfig::from_raw(rf, raw_config.update_interval, raw_config.update_retries)
            })
            .collect::<Vec<FeedConfig>>();

        if feeds.is_empty() {
            return Err(ConfigError::NoActiveFeeds);
        }

        Ok(Self {
            feeds,
            queue: raw_config.queue,
        })
    }
}

#[derive(Deserialize)]
struct RawConfig {
    /// Список конфигураций фидов
    pub feeds: Vec<RawFeedConfig>,

    /// Интервал обновлениия в секундах
    ///
    /// Значение по умолчанию: [`DEFAULT_UPDATE_INTERVAL`]
    pub update_interval: Option<i64>,

    /// Максимальное количество попыток
    ///
    /// Значение по умолчанию: [`DEFAULT_UPDATE_RETRIES`]
    pub update_retries: Option<usize>,

    /// Конфигурация очереди
    pub queue: QueueConfig,
}

impl RawConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        self.feeds.iter().try_for_each(|f| f.validate())?;

        if let Some(value) = self.update_interval {
            feed::validate_feed_update_interval(value)?;
        }

        if let Some(value) = self.update_retries {
            feed::validate_feed_update_retries(value)?;
        }

        self.queue.validate()?;

        Ok(())
    }
}

pub async fn load_config(path: impl AsRef<Path>) -> Result<Config, ConfigError> {
    let content = tokio::fs::read_to_string(path).await?;
    let raw_config: RawConfig = toml::from_str(&content)?;
    let config = raw_config.try_into()?;

    Ok(config)
}
