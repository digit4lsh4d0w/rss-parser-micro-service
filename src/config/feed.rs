use serde::Deserialize;

use crate::config::error::{FeedValidationError, FeedValidationResult};

const DEFAULT_UPDATE_INTERVAL: usize = 15 * 60;
const MIN_UPDATE_INTERVAL: usize = 5 * 60;

const DEFAULT_UPDATE_RETRIES: usize = 5;
const MIN_UPDATE_RETRIES: usize = 1;
const MAX_UPDATE_RETRIES: usize = 10;

/// Представляет RSS фид.
#[derive(Debug, Clone, Deserialize)]
pub struct FeedConfig {
    /// Название фида.
    pub name: Option<String>,

    /// Источник RSS фида.
    pub url: String,

    /// Интервал обновлениия в секундах.
    /// Значение по умолчанию: 15 минут.
    pub update_interval: Option<usize>,

    /// Максимальное количество попыток.
    /// Значение по умолчанию: 5 попыток.
    pub update_retries: Option<usize>,

    /// Активен ли фид.
    ///
    /// Варианты:
    ///
    /// - `true` - фид будет обрабатываться.
    /// - `false` - фид не будет обрабатываться.
    ///
    /// Значение по умолчанию: `true`
    pub active: Option<bool>,
}

impl FeedConfig {
    pub fn validate(&self) -> FeedValidationResult<()> {
        self.validate_url()?;
        self.validate_update_interval()?;
        self.validate_update_retries()?;
        Ok(())
    }

    fn validate_url(&self) -> FeedValidationResult<()> {
        if self.url.trim().is_empty() {
            return Err(FeedValidationError::EmptyUrl);
        }

        reqwest::Url::parse(&self.url).map_err(|_| FeedValidationError::InvalidUrlFormat)?;

        Ok(())
    }

    fn validate_update_interval(&self) -> FeedValidationResult<()> {
        if let Some(update_interval) = self.update_interval
            && update_interval < MIN_UPDATE_INTERVAL
        {
            return Err(FeedValidationError::UpdateIntervalTooSmall(
                MIN_UPDATE_INTERVAL,
                update_interval,
            ));
        }

        Ok(())
    }

    fn validate_update_retries(&self) -> FeedValidationResult<()> {
        if let Some(update_retries) = self.update_retries
            && update_retries < MIN_UPDATE_RETRIES
        {
            return Err(FeedValidationError::UpdateRetriesTooSmall(
                MIN_UPDATE_RETRIES,
                update_retries,
            ));
        }

        if let Some(update_retries) = self.update_retries
            && update_retries > MAX_UPDATE_RETRIES
        {
            return Err(FeedValidationError::UpdateRetriesTooBig(
                MAX_UPDATE_RETRIES,
                update_retries,
            ));
        }

        Ok(())
    }
}

impl Default for FeedConfig {
    fn default() -> Self {
        Self {
            name: None,
            url: "".to_string(),
            update_interval: Some(DEFAULT_UPDATE_INTERVAL),
            update_retries: Some(DEFAULT_UPDATE_RETRIES),
            active: Some(true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feed_empty_url() {
        let feed = FeedConfig {
            url: "".to_string(),
            ..Default::default()
        };

        assert_eq!(
            feed.validate_url().unwrap_err(),
            FeedValidationError::EmptyUrl
        );
    }

    #[test]
    fn test_feed_invalid_url() {
        let feed = FeedConfig {
            url: "not-a-url".to_string(),
            ..Default::default()
        };

        assert_eq!(
            feed.validate_url().unwrap_err(),
            FeedValidationError::InvalidUrlFormat
        );
    }

    #[test]
    fn test_feed_too_small_update_interval() {
        let feed = FeedConfig {
            update_interval: Some(5),
            ..Default::default()
        };

        assert_eq!(
            feed.validate_update_interval().unwrap_err(),
            FeedValidationError::UpdateIntervalTooSmall(MIN_UPDATE_INTERVAL, 5)
        );
    }

    #[test]
    fn test_feed_too_small_update_retries() {
        let feed = FeedConfig {
            update_retries: Some(0),
            ..Default::default()
        };

        assert_eq!(
            feed.validate_update_retries().unwrap_err(),
            FeedValidationError::UpdateRetriesTooSmall(MIN_UPDATE_RETRIES, 0)
        );
    }

    #[test]
    fn test_feed_too_big_update_retries() {
        let feed = FeedConfig {
            update_retries: Some(20),
            ..Default::default()
        };

        assert_eq!(
            feed.validate_update_retries().unwrap_err(),
            FeedValidationError::UpdateRetriesTooBig(MAX_UPDATE_RETRIES, 20)
        );
    }
}
