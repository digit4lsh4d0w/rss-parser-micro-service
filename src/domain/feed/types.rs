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
}
