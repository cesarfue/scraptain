use crate::constants::PlatformConfig;
use crate::error::{Result, ScraperError};
use crate::models::{Job, JobSearchParams, SelectorRule, SelectorType};
use reqwest::Client;
use scraper::{Html, Selector};
use url::Url;

pub struct PlatformScraper {
    client: Client,
    config: &'static PlatformConfig,
}

impl PlatformScraper {
    pub fn new(client: Client, config: &'static PlatformConfig) -> Self {
        Self { client, config }
    }

    pub fn hellowork(client: Client) -> Self {
        Self::new(client, &crate::constants::HELLOWORK)
    }

    pub async fn search(&self, params: JobSearchParams) -> Result<Vec<Job>> {
        let mut jobs: Vec<Job> = Vec::new();
        let limit = params.limit.unwrap_or(100);
        let url = self.build_search_url(&params)?;
        let response = self.client.get(&url).send().await?;
        if response.status().as_u16() == 429 {
            return Err(ScraperError::RateLimitExceeded);
        }
        if response.status().as_u16() == 403 {
            return Err(ScraperError::BlockedByTarget);
        }
        let html = response.text().await?;
        let document = Html::parse_document(&html);
        if let Some(job) = self.build_job(&document)? {
            jobs.push(job);
        }

        Ok(jobs)
    }

    fn build_job(&self, html: &Html) -> Result<Option<Job>> {
        let card = self.extract_from_rule(html, &self.config.selectors.job_card);
        println!("{:?}", card);

        Ok(Some(Job {
            job_id: String::new(),
            title: String::new(),
            company: String::new(),
            location: None,
            description: None,
            salary: None,
            url: String::new(),
            posted_date: None,
            job_type: None,
            experience_level: None,
            source: self.config.name.to_string(),
        }))
    }

    fn extract_from_rule(&self, document: &Html, rule: &SelectorRule) -> Option<String> {
        let selector = Selector::parse(rule.selector).ok()?;
        let elements: Vec<_> = document.select(&selector).collect();
        let index = rule.nth.unwrap_or(0);

        let element = elements.get(index)?;

        match &rule.selector_type {
            SelectorType::Text => Some(
                element
                    .text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim()
                    .to_string(),
            ),
            SelectorType::Attribute(attr) => element.value().attr(attr).map(|v| v.to_string()),
            SelectorType::Html => Some(element.html()),
        }
    }

    fn build_search_url(&self, params: &JobSearchParams) -> Result<String> {
        let url_params = &self.config.url_params;
        let mut url = Url::parse(self.config.url)?;

        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair(url_params.query, &params.query);
            if let Some(location) = &params.location {
                query_pairs.append_pair(url_params.location, location);
            }
            if let Some(radius) = params.radius {
                query_pairs.append_pair(url_params.radius, &radius.to_string());
            }
            if let Some(offset) = params.offset {
                query_pairs.append_pair(url_params.start, &offset.to_string());
            }
            if let Some(date_posted) = &params.date_posted {
                if let Some(seconds) = date_posted.to_seconds() {
                    query_pairs.append_pair(url_params.date_posted, &seconds.to_string());
                }
            }
        }

        Ok(url.to_string())
    }
}
