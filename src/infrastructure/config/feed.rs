use serde::Deserialize;

use crate::domain::feed::{
    Feed,
    errors::FeedError,
    types::{FeedName, FeedUpdateInterval, FeedUpdateRetries, FeedUrl},
};

#[derive(Deserialize)]
pub struct FeedDTO {
    pub name: String,
    pub url: String,
    pub update_interval: Option<u64>,
    pub update_retries: Option<usize>,
    pub active: Option<bool>,
}

impl FeedDTO {
    pub fn try_into_feed(
        self,
        update_interval: Option<u64>,
        update_retries: Option<usize>,
    ) -> Result<Feed, FeedError> {
        let name = FeedName::new(&self.name)?;
        let url = FeedUrl::new(&self.url)?;
        let update_interval = FeedUpdateInterval::new(self.update_interval.or(update_interval))?;
        let update_retries = FeedUpdateRetries::new(self.update_retries.or(update_retries))?;

        Ok(Feed::new(name, url, update_interval, update_retries))
    }
}
