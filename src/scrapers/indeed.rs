use crate::error::{Result, ScraperError};
use crate::models::{Job, JobSearchParams, JobSource};
use crate::scrapers::JobScraper;
use async_trait::async_trait;
use reqwest::Client;
use scraper::{Html, Selector};

pub struct IndeedScraper {
    client: Client,
}

impl IndeedScraper {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    fn build_search_url(&self, params: &JobSearchParams) -> Result<String> {
        let base_url = "https://www.indeed.com/jobs";
        let mut url = url::Url::parse(base_url)?;

        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("q", &params.query);

            if let Some(location) = &params.location {
                query_pairs.append_pair("l", location);
            }

            if let Some(radius) = params.radius {
                query_pairs.append_pair("radius", &radius.to_string());
            }

            if let Some(offset) = params.offset {
                query_pairs.append_pair("start", &offset.to_string());
            }
        }

        Ok(url.to_string())
    }

    fn parse_jobs(&self, html: &str) -> Result<Vec<Job>> {
        let document = Html::parse_document(html);
        let mut jobs = Vec::new();

        // TODO: Implement actual HTML parsing logic
        // This is a placeholder structure - you'll need to inspect Indeed's HTML
        // and update selectors accordingly

        // Example selectors (these will need to be adjusted):
        let job_card_selector = Selector::parse(".job_seen_beacon")
            .map_err(|e| ScraperError::ParseError(format!("Invalid selector: {:?}", e)))?;

        for element in document.select(&job_card_selector) {
            // Extract job details
            // This is where you'll parse title, company, location, etc.

            // Placeholder job structure
            let job = Job {
                title: String::from("TODO: Extract title"),
                company: String::from("TODO: Extract company"),
                location: Some(String::from("TODO: Extract location")),
                description: None,
                salary: None,
                url: String::from("TODO: Extract URL"),
                posted_date: None,
                job_type: None,
                experience_level: None,
                source: JobSource::Indeed,
                job_id: String::from("TODO: Extract job ID"),
            };

            jobs.push(job);
        }

        if jobs.is_empty() {
            return Err(ScraperError::NoJobsFound);
        }

        Ok(jobs)
    }
}

#[async_trait]
impl JobScraper for IndeedScraper {
    async fn search(&self, params: JobSearchParams) -> Result<Vec<Job>> {
        let url = self.build_search_url(&params)?;

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(ScraperError::RequestFailed(
                response.error_for_status().unwrap_err(),
            ));
        }

        let html = response.text().await?;
        self.parse_jobs(&html)
    }

    fn name(&self) -> &str {
        "Indeed"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_indeed_scraper() {
        let client = Client::new();
        let scraper = IndeedScraper::new(client);

        let params = JobSearchParams {
            query: "software engineer".to_string(),
            location: Some("San Francisco".to_string()),
            ..Default::default()
        };

        let result = scraper.search(params).await;
        assert!(result.is_ok() || matches!(result, Err(ScraperError::NoJobsFound)));
    }
}
