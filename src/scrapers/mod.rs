pub mod indeed;
pub mod linkedin;

use crate::error::{Result, ScraperError};
use crate::models::{Job, JobSearchParams, Selectors};
use async_trait::async_trait;
use scraper::{ElementRef, Selector};

#[async_trait]
pub trait PlatformScraper: Send + Sync {
    async fn search(&self, params: JobSearchParams) -> Result<Vec<Job>>;
    fn name(&self) -> &str;
    fn selectors() -> &'static Selectors;

    fn parse_selector(selector: &str) -> Result<Selector> {
        Selector::parse(selector).map_err(|e| {
            ScraperError::ParseError(format!("Invalid selector '{}': {:?}", selector, e))
        })
    }

    fn extract_text(element: ElementRef) -> String {
        element.text().collect::<String>().trim().to_string()
    }

    fn extract_attr<'a>(element: &ElementRef<'a>, attr: &str) -> Option<&'a str> {
        element.value().attr(attr)
    }

    fn is_remote_job(title: &str, description: Option<&str>, location: Option<&str>) -> bool {
        let remote_keywords = ["remote", "work from home", "wfh", "telecommute"];

        let full_text = format!(
            "{} {} {}",
            title,
            description.unwrap_or(""),
            location.unwrap_or("")
        )
        .to_lowercase();

        remote_keywords
            .iter()
            .any(|keyword| full_text.contains(keyword))
    }
}
