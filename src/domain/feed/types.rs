use std::time::Duration;

use url::Url;

use crate::domain::feed::errors::FeedError;

/// Название фида
#[derive(Debug, Clone)]
pub struct FeedName(String);

impl FeedName {
    pub fn new(name: &str) -> Result<Self, FeedError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(FeedError::EmptyName);
        }

        Ok(Self(name.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Источник RSS фида
#[derive(Debug, Clone)]
pub struct FeedUrl(Url);

impl FeedUrl {
    pub fn new(url: &str) -> Result<Self, FeedError> {
        let url = Url::parse(url.trim())?;

        Ok(Self(url))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Интервал обновлениия в секундах
///
/// Значение по умолчанию: [`Self::DEFAULT_MINUTES`]
#[derive(Debug, Clone)]
pub struct FeedUpdateInterval(Duration);

impl FeedUpdateInterval {
    pub const DEFAULT_MINUTES: u64 = 15;
    pub const MIN_MINUTES: u64 = 5;

    pub fn new(minutes: Option<u64>) -> Result<Self, FeedError> {
        let minutes = minutes.unwrap_or(Self::DEFAULT_MINUTES);
        if minutes < Self::MIN_MINUTES {
            return Err(FeedError::InvalidUpdateInterval {
                min: Self::MIN_MINUTES,
                got: minutes,
            });
        }

        Ok(Self(Duration::from_mins(minutes)))
    }

    pub fn as_secs(&self) -> u64 {
        self.0.as_secs()
    }
}

/// Максимальное количество попыток
///
/// Значение по умолчанию: [`Self::DEFAULT`]
#[derive(Debug, Clone)]
pub struct FeedUpdateRetries(usize);

impl FeedUpdateRetries {
    pub const DEFAULT: usize = 3;
    pub const MIN: usize = 1;
    pub const MAX: usize = 10;

    pub fn new(retries: Option<usize>) -> Result<Self, FeedError> {
        let retries = retries.unwrap_or(Self::DEFAULT);
        if !(Self::MIN..=Self::MAX).contains(&retries) {
            return Err(FeedError::InvalidUpdateRetries {
                min: Self::MIN,
                max: Self::MAX,
                got: retries,
            });
        }

        Ok(Self(retries))
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// FeedName tests
    mod feed_name {
        use super::*;

        #[test]
        fn test_valid_name() {
            let name = FeedName::new("Test Feed").unwrap();
            assert_eq!(name.as_str(), "Test Feed");
        }

        #[test]
        fn test_name_trims_whitespace() {
            let name = FeedName::new("    Test Feed    ").unwrap();
            assert_eq!(name.as_str(), "Test Feed");
        }

        #[test]
        fn test_empty_name_returns_error() {
            let result = FeedName::new("");
            assert!(result.is_err());
            assert!(matches!(result.unwrap_err(), FeedError::EmptyName));
        }

        #[test]
        fn test_whitespace_only_name_returns_error() {
            let result = FeedName::new("    ");
            assert!(result.is_err());
            assert!(matches!(result.unwrap_err(), FeedError::EmptyName));
        }
    }

    /// FeedUrl tests
    mod feed_url {
        use super::*;

        #[test]
        fn test_valid_url() {
            let url = FeedUrl::new("https://example.com/feed.xml").unwrap();
            assert_eq!(url.as_str(), "https://example.com/feed.xml");
        }

        #[test]
        fn test_url_trims_whitespace() {
            let url = FeedUrl::new("    https://example.com/feed.xml    ").unwrap();
            assert_eq!(url.as_str(), "https://example.com/feed.xml");
        }

        #[test]
        fn test_invalid_url_returns_error() {
            let result = FeedUrl::new("not-a-valid-url");
            assert!(result.is_err());
        }
    }

    /// FeedUpdateInterval tests
    mod feed_update_interval {
        use super::*;

        #[test]
        fn test_default_value() {
            let interval = FeedUpdateInterval::new(None).unwrap();
            assert_eq!(interval.as_secs(), 15 * 60);
        }

        #[test]
        fn test_minimum_value() {
            let interval = FeedUpdateInterval::new(Some(5)).unwrap();
            assert_eq!(interval.as_secs(), 5 * 60);
        }

        #[test]
        fn test_valid_custom_value() {
            let interval = FeedUpdateInterval::new(Some(30)).unwrap();
            assert_eq!(interval.as_secs(), 30 * 60);
        }

        #[test]
        fn test_below_minimum_returns_error() {
            let result = FeedUpdateInterval::new(Some(4));
            assert!(result.is_err());

            if let Err(FeedError::InvalidUpdateInterval { min, got }) = result {
                assert_eq!(min, 5);
                assert_eq!(got, 4);
            } else {
                panic!("Expected InvalidUpdateInterval error");
            }
        }
    }

    /// FeedUpdateRetries tests
    mod feed_update_retries {
        use super::*;

        #[test]
        fn test_default_value() {
            let retries = FeedUpdateRetries::new(None).unwrap();
            assert_eq!(retries.as_usize(), 3);
        }

        #[test]
        fn test_minimum_value() {
            let retries = FeedUpdateRetries::new(Some(1)).unwrap();
            assert_eq!(retries.as_usize(), 1);
        }

        #[test]
        fn test_maximum_value() {
            let retries = FeedUpdateRetries::new(Some(10)).unwrap();
            assert_eq!(retries.as_usize(), 10);
        }

        #[test]
        fn test_middle_value() {
            let retries = FeedUpdateRetries::new(Some(5)).unwrap();
            assert_eq!(retries.as_usize(), 5);
        }

        #[test]
        fn test_below_minimum_returns_error() {
            let result = FeedUpdateRetries::new(Some(0));
            assert!(result.is_err());
            if let Err(FeedError::InvalidUpdateRetries { min, max, got }) = result {
                assert_eq!(min, 1);
                assert_eq!(max, 10);
                assert_eq!(got, 0);
            } else {
                panic!("Expected InvalidUpdateRetries error");
            }
        }

        #[test]
        fn test_above_maximum_returns_error() {
            let result = FeedUpdateRetries::new(Some(11));
            assert!(result.is_err());
            if let Err(FeedError::InvalidUpdateRetries { min, max, got }) = result {
                assert_eq!(min, 1);
                assert_eq!(max, 10);
                assert_eq!(got, 11);
            } else {
                panic!("Expected InvalidUpdateRetries error");
            }
        }
    }
}
