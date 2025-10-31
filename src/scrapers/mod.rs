pub mod indeed;
pub mod linkedin;

use crate::error::{Result, ScraperError};
use crate::models::{Job, JobSearchParams};
use async_trait::async_trait;
use scraper::{ElementRef, Selector};

#[async_trait]
pub trait PlatformScraper: Send + Sync {
    async fn search(&self, params: JobSearchParams) -> Result<Vec<Job>>;
    fn name(&self) -> &str;

    fn parse_selector(selector: &str) -> Result<Selector> {
        Selector::parse(selector).map_err(|e| {
            ScraperError::ParseError(format!("Invalid selector '{}': {:?}", selector, e))
        })
    }

    fn extract_text(element: ElementRef) -> String {
        // Remove 'pub'
        element.text().collect::<String>().trim().to_string()
    }

    fn extract_attr<'a>(element: &ElementRef<'a>, attr: &str) -> Option<&'a str> {
        // Remove 'pub'
        element.value().attr(attr)
    }

    fn is_remote_job(title: &str, description: Option<&str>, location: Option<&str>) -> bool {
        // Remove 'pub'
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

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyScraper;

    #[async_trait]
    impl PlatformScraper for DummyScraper {
        async fn search(&self, _params: JobSearchParams) -> Result<Vec<Job>> {
            Ok(vec![])
        }
        fn name(&self) -> &str {
            "Dummy"
        }
    }

    #[test]
    fn test_is_remote_job() {
        assert!(DummyScraper::is_remote_job("Remote Developer", None, None));
        assert!(DummyScraper::is_remote_job(
            "Developer",
            Some("Work from home"),
            None
        ));
        assert!(DummyScraper::is_remote_job(
            "Developer",
            None,
            Some("Remote, USA")
        ));
        assert!(!DummyScraper::is_remote_job(
            "Developer",
            Some("Office based"),
            Some("New York")
        ));
    }
}
