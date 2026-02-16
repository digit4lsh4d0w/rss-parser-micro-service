use crate::domain::feed::types::FeedUrl;
use bytes::Bytes;
use errors::FetcherError;

pub mod errors;

#[derive(Debug)]
pub struct FetchResults<'a> {
    pub successful: Vec<(&'a FeedUrl, Bytes)>,
    pub failed: Vec<(&'a FeedUrl, FetcherError)>,
}

pub trait RssFetcher<'a>: Send + Sync {
    fn fetch(&self, url: &FeedUrl) -> impl Future<Output = Result<Bytes, FetcherError>> + Send;
    fn fetch_all(&'a self, urls: &'a [FeedUrl]) -> impl Future<Output = FetchResults<'a>> + Send;
}
