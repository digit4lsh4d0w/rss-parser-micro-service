use chrono::Duration;
use serde::Deserialize;

use crate::config::error::FeedError;

/// Стандартный интервал опроса фидов - 15 минут
pub const DEFAULT_UPDATE_INTERVAL: i64 = 15 * 60;
/// Минимальный интервал опроса фидов - 5 минут
pub const MIN_UPDATE_INTERVAL: i64 = 5 * 60;
/// Стандартное количество попыток опроса фида
pub const DEFAULT_UPDATE_RETRIES: usize = 3;
/// Минимальное количество попыток опроса фида
pub const MIN_UPDATE_RETRIES: usize = 1;
/// Максимальное количество попыток опроса фида
pub const MAX_UPDATE_RETRIES: usize = 10;

/// Конфигурация источника RSS фида
#[derive(Debug)]
pub struct FeedConfig {
    /// Название фида
    pub name: String,

    /// Источник RSS фида
    pub url: String,

    /// Интервал обновлениия в секундах
    ///
    /// Значение по умолчанию: [`DEFAULT_UPDATE_INTERVAL`]
    pub update_interval: Duration,

    /// Максимальное количество попыток
    ///
    /// Значение по умолчанию: [`DEFAULT_UPDATE_RETRIES`]
    pub update_retries: usize,
}

impl FeedConfig {
    pub fn from_raw(
        raw_feed: RawFeedConfig,
        update_interval: Option<i64>,
        update_retries: Option<usize>,
    ) -> FeedConfig {
        let update_interval = Duration::seconds(
            raw_feed
                .update_interval
                .or(update_interval)
                .unwrap_or(DEFAULT_UPDATE_INTERVAL),
        );

        let update_retries = raw_feed
            .update_retries
            .or(update_retries)
            .unwrap_or(DEFAULT_UPDATE_RETRIES);

        FeedConfig {
            name: raw_feed.name,
            url: raw_feed.url,
            update_interval,
            update_retries,
        }
    }
}

#[derive(Deserialize)]
pub struct RawFeedConfig {
    pub name: String,
    pub url: String,
    pub update_interval: Option<i64>,
    pub update_retries: Option<usize>,
    pub active: Option<bool>,
}

impl RawFeedConfig {
    pub fn validate(&self) -> Result<(), FeedError> {
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

pub fn validate_feed_name(name: &str) -> Result<(), FeedError> {
    if name.trim().is_empty() {
        return Err(FeedError::EmptyName);
    }

    Ok(())
}

pub fn validate_url(url: &str) -> Result<(), FeedError> {
    if url.trim().is_empty() {
        return Err(FeedError::EmptyUrl);
    }

    reqwest::Url::parse(url).map_err(FeedError::InvalidUrl)?;

    Ok(())
}

pub fn validate_feed_update_interval(interval: i64) -> Result<(), FeedError> {
    if interval < MIN_UPDATE_INTERVAL {
        return Err(FeedError::InvalidUpdateInterval(interval));
    }

    Ok(())
}

pub fn validate_feed_update_retries(retries: usize) -> Result<(), FeedError> {
    if !(MIN_UPDATE_RETRIES..=MAX_UPDATE_RETRIES).contains(&retries) {
        return Err(FeedError::InvalidUpdateRetries(retries));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_feed_name() {
        let result = validate_feed_name("");
        assert!(matches!(result, Err(FeedError::EmptyName)));
    }

    #[test]
    fn test_valid_feed_name() {
        let result = validate_feed_name("feed");
        assert!(matches!(result, Ok(())));
    }

    #[test]
    fn test_empty_url() {
        let result = validate_url("");
        assert!(matches!(result, Err(FeedError::EmptyUrl)));
    }

    #[test]
    fn test_invalid_url() {
        let result = validate_url("not-a-url");
        assert!(matches!(result, Err(FeedError::InvalidUrl(_))));
    }

    #[test]
    fn test_valid_url() {
        let result = validate_url("http://localhost");
        assert!(matches!(result, Ok(())));
    }

    #[test]
    fn test_feed_too_small_update_interval() {
        const INVALID_UPDATE_INTERVAL: i64 = 5;
        let result = validate_feed_update_interval(INVALID_UPDATE_INTERVAL);
        assert!(matches!(
            result,
            Err(FeedError::InvalidUpdateInterval(INVALID_UPDATE_INTERVAL))
        ));
    }

    #[test]
    fn test_feed_valid_update_interval() {
        let result = validate_feed_update_interval(MIN_UPDATE_INTERVAL);
        assert!(matches!(result, Ok(())));
    }

    #[test]
    fn test_feed_too_small_update_retries() {
        const INVALID_UPDATE_RETRIES: usize = 0;
        let result = validate_feed_update_retries(INVALID_UPDATE_RETRIES);
        assert!(matches!(
            result,
            Err(FeedError::InvalidUpdateRetries(INVALID_UPDATE_RETRIES))
        ));
    }

    #[test]
    fn test_feed_too_big_update_retries() {
        const INVALID_UPDATE_RETRIES: usize = 20;
        let result = validate_feed_update_retries(INVALID_UPDATE_RETRIES);
        assert!(matches!(
            result,
            Err(FeedError::InvalidUpdateRetries(INVALID_UPDATE_RETRIES))
        ));
    }
}
