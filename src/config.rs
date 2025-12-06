use std::path::Path;

use serde::Deserialize;

use feed::FeedConfig;

use crate::config::{
    error::ConfigResult,
    feed::{DEFAULT_UPDATE_INTERVAL, DEFAULT_UPDATE_RETRIES, RawFeedConfig},
};

mod error;
mod feed;
mod notification;

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

/// Основные настройки приложения
#[derive(Debug)]
pub struct Config {
    /// Список фидов
    pub feeds: Vec<FeedConfig>,
    // pub notification_service: NotificationServiceConfig,
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw_config = RawConfig::deserialize(deserializer)?;

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

        Ok(Config { feeds })
    }
}

pub async fn load_config(path: impl AsRef<Path>) -> ConfigResult<Config> {
    let content = tokio::fs::read_to_string(path).await?;
    let config = toml::from_str(&content)?;
    Ok(config)
}
