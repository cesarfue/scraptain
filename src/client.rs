use crate::error::Result;
use crate::models::{Job, JobSearchParams};
use crate::PlatformScraper;
use futures::future::join_all;
use reqwest::Client;
use std::time::Duration;

pub struct ScraperClient {
    scraper: PlatformScraper,
}

impl ScraperClient {
    pub fn new() -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
            .build()
            .expect("Failed to build HTTP client");

        let scraper = PlatformScraper::new(http_client);

        Self { scraper }
    }

    pub async fn search_all(&self, params: JobSearchParams) -> Result<Vec<Job>> {
        let mut all_jobs = Vec::new();

        let tasks = self
            .scrapers
            .iter()
            .map(|scraper| scraper.search(params.clone()))
            .collect::<Vec<_>>();

        let results = join_all(tasks).await;

        for result in results {
            if let Ok(mut jobs) = result {
                all_jobs.append(&mut jobs);
            }
        }

        Ok(all_jobs)
    }
}

impl Default for ScraperClient {
    fn default() -> Self {
        Self::new()
    }
}
