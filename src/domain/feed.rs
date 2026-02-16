use crate::domain::feed::types::{FeedName, FeedUpdateInterval, FeedUpdateRetries, FeedUrl};

pub mod errors;
pub mod types;

/// RSS фид
#[derive(Debug, Clone)]
pub struct Feed {
    name: FeedName,
    url: FeedUrl,
    update_interval: FeedUpdateInterval,
    update_retries: FeedUpdateRetries,
}

impl Feed {
    pub fn new(
        name: FeedName,
        url: FeedUrl,
        update_interval: FeedUpdateInterval,
        update_retries: FeedUpdateRetries,
    ) -> Self {
        Self {
            name,
            url,
            update_interval,
            update_retries,
        }
    }
}
