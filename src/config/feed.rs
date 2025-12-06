use serde::Deserialize;

use crate::config::{
    error::ValidationResult,
    validation::{
        validate_feed_name, validate_update_interval, validate_update_retries, validate_url,
    },
};

pub const DEFAULT_UPDATE_INTERVAL: usize = 15 * 60;
pub const MIN_UPDATE_INTERVAL: usize = 5 * 60;

pub const DEFAULT_UPDATE_RETRIES: usize = 3;
pub const MIN_UPDATE_RETRIES: usize = 1;
pub const MAX_UPDATE_RETRIES: usize = 10;

/// RSS фид.
#[derive(Debug)]
pub struct FeedConfig {
    /// Название фида.
    pub name: String,

    /// Источник RSS фида.
    pub url: String,

    /// Интервал обновлениия в секундах.
    ///
    /// Значение по умолчанию: [`DEFAULT_UPDATE_INTERVAL`].
    pub update_interval: usize,

    /// Максимальное количество попыток.
    ///
    /// Значение по умолчанию: [`DEFAULT_UPDATE_RETRIES`].
    pub update_retries: usize,
}

#[derive(Deserialize)]
pub struct RawFeedConfig {
    pub name: String,
    pub url: String,
    pub update_interval: Option<usize>,
    pub update_retries: Option<usize>,
    pub active: Option<bool>,
}

impl RawFeedConfig {
    pub fn validate(&self) -> ValidationResult<()> {
        validate_feed_name(&self.name)?;
        validate_url(&self.url)?;
        if let Some(value) = self.update_interval {
            validate_update_interval(value)?;
        }
        if let Some(value) = self.update_retries {
            validate_update_retries(value)?;
        }

        Ok(())
    }
}

impl Default for RawFeedConfig {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            url: "".to_string(),
            update_interval: Some(DEFAULT_UPDATE_INTERVAL),
            update_retries: Some(DEFAULT_UPDATE_RETRIES),
            active: Some(true),
        }
    }
}
