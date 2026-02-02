use crate::domain::feed::types::{FeedName, FeedUpdateInterval, FeedUpdateRetries, FeedUrl};

pub mod errors;
pub mod types;

/// RSS фид
#[derive(Debug, Clone)]
pub struct Feed {
    pub name: FeedName,
    pub url: FeedUrl,
    pub update_interval: FeedUpdateInterval,
    pub update_retries: FeedUpdateRetries,
}
