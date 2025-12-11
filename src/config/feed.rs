use serde::Deserialize;

use crate::config::{
    error::ValidationResult,
    validation::{
        validate_feed_name, validate_feed_update_interval, validate_feed_update_retries,
        validate_url,
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

impl FeedConfig {
    pub fn try_from_raw_feed_config(
        raw_feed: RawFeedConfig,
        update_interval: Option<usize>,
        update_retries: Option<usize>,
    ) -> ValidationResult<FeedConfig> {
        raw_feed.validate()?;

        Ok(FeedConfig {
            name: raw_feed.name,
            url: raw_feed.url,
            update_interval: raw_feed
                .update_interval
                .or(update_interval)
                .unwrap_or(DEFAULT_UPDATE_INTERVAL),
            update_retries: raw_feed
                .update_retries
                .or(update_retries)
                .unwrap_or(DEFAULT_UPDATE_RETRIES),
        })
    }
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
            validate_feed_update_interval(value)?;
        }
        if let Some(value) = self.update_retries {
            validate_feed_update_retries(value)?;
        }

        Ok(())
    }
}
