pub mod hellowork;
pub mod linkedin;

use crate::error::{Result, ScraperError};
use crate::models::{
    DatePosted, ExperienceLevel, Job, JobSearchParams, JobType, Selectors, UrlParameters,
};
use scraper::{ElementRef, Selector};
use url::Url;

#[async_trait::async_trait(?Send)]
pub trait PlatformScraper: Send + Sync {
    async fn search(&self, params: JobSearchParams) -> Result<Vec<Job>>;

    fn name(&self) -> &str;

    fn selectors(&self) -> &'static Selectors;

    fn base_url(&self) -> &'static str;

    fn url_params(&self) -> &'static UrlParameters;

    fn search_endpoint(&self) -> &'static str {
        "/jobs"
    }

    fn build_search_url(&self, params: &JobSearchParams, start: u32) -> Result<String> {
        let full_url = format!("{}{}", self.base_url(), self.search_endpoint());
        let mut url = Url::parse(&full_url)?;
        let url_params = self.url_params();

        {
            let mut query_pairs = url.query_pairs_mut();

            query_pairs.append_pair(url_params.query, &params.query);

            if let Some(location) = &params.location {
                query_pairs.append_pair(url_params.location, location);
            }

            if let Some(radius) = params.radius {
                query_pairs.append_pair(url_params.radius, &radius.to_string());
            }

            if start > 0 {
                query_pairs.append_pair(url_params.start, &start.to_string());
            }

            if let Some(ref job_type) = params.job_type {
                if let Some(code) = self.job_type_code(job_type) {
                    query_pairs.append_pair(url_params.job_type, code);
                }
            }

            if let Some(ref experience_level) = params.experience_level {
                if let Some(code) = self.experience_level_code(experience_level) {
                    query_pairs.append_pair(url_params.experience_level, code);
                }
            }

            if let Some(ref date_posted) = params.date_posted {
                if let Some(value) = self.date_posted_value(date_posted) {
                    query_pairs.append_pair(url_params.date_posted, &value);
                }
            }
        }

        Ok(url.to_string())
    }

    fn job_type_code(&self, _job_type: &JobType) -> Option<&'static str> {
        None
    }

    fn experience_level_code(&self, _level: &ExperienceLevel) -> Option<&'static str> {
        None
    }

    fn date_posted_value(&self, _date: &DatePosted) -> Option<String> {
        None
    }

    fn parse_selector(&self, selector: &str) -> Result<Selector> {
        Selector::parse(selector).map_err(|e| {
            ScraperError::ParseError(format!("Invalid selector '{}': {:?}", selector, e))
        })
    }

    fn extract_text(&self, element: ElementRef) -> String {
        element.text().collect::<String>().trim().to_string()
    }

    fn extract_attr<'a>(&self, element: &ElementRef<'a>, attr: &str) -> Option<&'a str> {
        element.value().attr(attr)
    }
}
