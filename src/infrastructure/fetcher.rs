use bytes::Bytes;
use futures::future::join_all;
use reqwest::Client;
use tokio::sync::Semaphore;
use tracing::{debug, info, instrument};

use crate::{
    application::fetcher::{FetchResults, RssFetcher, errors::FetcherError},
    domain::feed::types::FeedUrl,
};

// TODO: Вынести в переменную конфигурации
const MAX_CONCURRENT_REQUESTS: usize = 4;

pub struct HttpRssFetcher {
    client: Client,
    semaphore: Semaphore,
}

impl HttpRssFetcher {
    pub fn new(client: Client) -> Self {
        info!(
            max_concurrent = MAX_CONCURRENT_REQUESTS,
            "Creating HttpRssFetcher"
        );

        Self {
            client,
            // TODO: Заменить на значение из конфига
            semaphore: Semaphore::new(MAX_CONCURRENT_REQUESTS),
        }
    }
}

impl RssFetcher<'_> for HttpRssFetcher {
    #[instrument(skip(self), fields(url = %url.as_str()))]
    async fn fetch(&self, url: &FeedUrl) -> Result<Bytes, FetcherError> {
        debug!("Sending HTTP request");
        let response = self.client.get(url.as_str()).send().await.map_err(|e| {
            tracing::warn!(error = %e, "HTTP request failed");
            // TODO: Изменить заглушку на реальный тип
            todo!();
        })?;

        let status = response.status();
        debug!(status = %status, "Received response");

        let bytes = response.bytes().await.map_err(|e| {
            tracing::warn!(error = %e, "Failed to read response body");
            // TODO: Изменить заглушку на реальный тип
            todo!();
        })?;

        debug!(size_bytes = bytes.len(), "Successfully fetched feed");
        Ok(bytes)
    }

    #[instrument(skip(self, urls), fields(total_urls = urls.len()))]
    async fn fetch_all<'a>(&self, urls: &'a [FeedUrl]) -> FetchResults<'a> {
        info!("Starting batch fetch");

        let futures = urls.iter().map(|url| async move {
            let _permit = self.semaphore.acquire().await.unwrap();
            debug!(url = %url.as_str(), "Acquired semaphore permit");

            let result = self.fetch(url).await;
            (url, result)
        });

        let results = join_all(futures).await;

        let mut successful = Vec::with_capacity(urls.len());
        let mut failed = Vec::new();

        for (url, result) in results {
            match result {
                Ok(bytes) => successful.push((url, bytes)),
                Err(e) => failed.push((url, e)),
            }
        }

        info!(
            successful = successful.len(),
            failed = failed.len(),
            "Finished batch fetch"
        );

        FetchResults { successful, failed }
    }
}
