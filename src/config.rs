use std::path::Path;

use anyhow::Result;
use serde::Deserialize;

use feed::FeedConfig;

mod error;
mod feed;
mod notification;

/// Основные настройки приложения
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Список фидов
    pub feeds: Vec<FeedConfig>,

    /// Интервал обновлениия в секундах.
    /// Значение по умолчанию: 15 минут.
    pub update_interval: Option<usize>,

    /// Максимальное количество попыток.
    /// Значение по умолчанию: 5 попыток.
    pub update_retries: Option<usize>,
    // pub notification_service: NotificationServiceConfig,
}

pub async fn load_config(path: impl AsRef<Path>) -> Result<Config> {
    let content = tokio::fs::read_to_string(path).await?;
    let config = toml::from_str(&content)?;
    Ok(config)
}
