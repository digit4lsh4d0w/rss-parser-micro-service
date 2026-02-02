use std::path::Path;

use serde::Deserialize;

use error::ConfigError;
use feed::FeedDTO;
use queue::QueueConfig;

use crate::domain::feed::{Feed, errors::FeedError};

pub mod error;
mod feed;
mod queue;

/// Основные настройки приложения
#[derive(Debug)]
pub struct Config {
    /// Список конфигураций фидов
    pub feeds: Vec<Feed>,

    /// Конфигурация очереди
    pub queue: QueueConfig,
}

impl TryFrom<ConfigDTO> for Config {
    type Error = ConfigError;

    fn try_from(config: ConfigDTO) -> Result<Self, Self::Error> {
        let feeds = config
            .feeds
            .into_iter()
            // Фильтрация неактивных фидов
            .filter(|rf| rf.active.unwrap_or(true))
            // Преобразование в доменную модель [`Feed`]
            .map(|rf| rf.try_into_feed(config.update_interval, config.update_retries))
            .collect::<Result<Vec<Feed>, FeedError>>()?;

        if feeds.is_empty() {
            return Err(ConfigError::NoActiveFeeds);
        }

        Ok(Self {
            feeds,
            queue: config.queue,
        })
    }
}

#[derive(Deserialize)]
struct ConfigDTO {
    /// Список конфигураций фидов
    pub feeds: Vec<FeedDTO>,

    /// Интервал обновлениия в секундах
    ///
    /// Значение по умолчанию: [`DEFAULT_UPDATE_INTERVAL`]
    pub update_interval: Option<u64>,

    /// Максимальное количество попыток
    ///
    /// Значение по умолчанию: [`DEFAULT_UPDATE_RETRIES`]
    pub update_retries: Option<usize>,

    /// Конфигурация очереди
    pub queue: QueueConfig,
}

pub async fn load_config(path: impl AsRef<Path>) -> Result<Config, ConfigError> {
    let content = tokio::fs::read_to_string(path).await?;
    let raw_config: ConfigDTO = toml::from_str(&content)?;
    let config = raw_config.try_into()?;

    Ok(config)
}
