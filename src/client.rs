use crate::error::Result;
use crate::models::{Job, JobSearchParams};
use crate::scrapers::{indeed::IndeedScraper, linkedin::LinkedInScraper, PlatformScraper};
use reqwest::Client;
use std::time::Duration;

pub struct ScraperClient {
    linkedin_scraper: LinkedInScraper,
    indeed_scraper: IndeedScraper,
}

impl ScraperClient {
    pub fn new() -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .build()
            .expect("Failed to build HTTP client");

        Self {
            linkedin_scraper: LinkedInScraper::new(http_client.clone()),
            indeed_scraper: IndeedScraper::new(http_client),
        }
    }

    pub fn with_client(http_client: Client) -> Self {
        Self {
            linkedin_scraper: LinkedInScraper::new(http_client.clone()),
            indeed_scraper: IndeedScraper::new(http_client),
        }
    }

    pub async fn search_linkedin(&self, params: JobSearchParams) -> Result<Vec<Job>> {
        self.linkedin_scraper.search(params).await
    }

    pub async fn search_indeed(&self, params: JobSearchParams) -> Result<Vec<Job>> {
        self.indeed_scraper.search(params).await
    }

    pub async fn search_all(&self, params: JobSearchParams) -> Result<Vec<Job>> {
        let mut all_jobs = Vec::new();

        let (linkedin_result, indeed_result) = tokio::join!(
            self.search_linkedin(params.clone()),
            self.search_indeed(params)
        );
        if let Ok(mut linkedin_jobs) = linkedin_result {
            all_jobs.append(&mut linkedin_jobs);
        }

        if let Ok(mut indeed_jobs) = indeed_result {
            all_jobs.append(&mut indeed_jobs);
        }

        Ok(all_jobs)
    }
}

impl Default for ScraperClient {
    fn default() -> Self {
        Self::new()
    }
}
