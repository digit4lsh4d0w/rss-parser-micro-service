use serde::Deserialize;

use crate::config::error::{FeedValidationError, FeedValidationResult};

pub const DEFAULT_UPDATE_INTERVAL: usize = 15 * 60;
pub const MIN_UPDATE_INTERVAL: usize = 5 * 60;

pub const DEFAULT_UPDATE_RETRIES: usize = 3;
pub const MIN_UPDATE_RETRIES: usize = 1;
pub const MAX_UPDATE_RETRIES: usize = 10;

#[derive(Deserialize)]
pub(crate) struct RawFeedConfig {
    pub name: String,
    pub url: String,
    pub update_interval: Option<usize>,
    pub update_retries: Option<usize>,
    pub active: Option<bool>,
}

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
    pub fn validate(&self) -> FeedValidationResult<()> {
        self.validate_name()?;
        self.validate_url()?;
        self.validate_update_interval()?;
        self.validate_update_retries()?;

        Ok(())
    }

    fn validate_name(&self) -> FeedValidationResult<()> {
        if self.name.trim().is_empty() {
            return Err(FeedValidationError::EmptyName);
        }

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
        if self.update_interval < MIN_UPDATE_INTERVAL {
            return Err(FeedValidationError::UpdateIntervalTooSmall(
                self.update_interval,
            ));
        }

        Ok(())
    }

    fn validate_update_retries(&self) -> FeedValidationResult<()> {
        if self.update_retries < MIN_UPDATE_RETRIES {
            return Err(FeedValidationError::UpdateRetriesTooSmall(
                self.update_retries,
            ));
        }

        if self.update_retries > MAX_UPDATE_RETRIES {
            return Err(FeedValidationError::UpdateRetriesTooBig(
                self.update_retries,
            ));
        }

        Ok(())
    }
}

impl Default for FeedConfig {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            url: "".to_string(),
            update_interval: DEFAULT_UPDATE_INTERVAL,
            update_retries: DEFAULT_UPDATE_RETRIES,
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
        const INVALID_UPDATE_INTERVAL: usize = 5;
        let feed = FeedConfig {
            update_interval: INVALID_UPDATE_INTERVAL,
            ..Default::default()
        };

        assert_eq!(
            feed.validate_update_interval().unwrap_err(),
            FeedValidationError::UpdateIntervalTooSmall(INVALID_UPDATE_INTERVAL)
        );
    }

    #[test]
    fn test_feed_too_small_update_retries() {
        const INVALID_UPDATE_RETRIES: usize = 0;
        let feed = FeedConfig {
            update_retries: INVALID_UPDATE_RETRIES,
            ..Default::default()
        };

        assert_eq!(
            feed.validate_update_retries().unwrap_err(),
            FeedValidationError::UpdateRetriesTooSmall(INVALID_UPDATE_RETRIES)
        );
    }

    #[test]
    fn test_feed_too_big_update_retries() {
        const INVALID_UPDATE_RETRIES: usize = 20;
        let feed = FeedConfig {
            update_retries: INVALID_UPDATE_RETRIES,
            ..Default::default()
        };

        assert_eq!(
            feed.validate_update_retries().unwrap_err(),
            FeedValidationError::UpdateRetriesTooBig(INVALID_UPDATE_RETRIES)
        );
    }
}
