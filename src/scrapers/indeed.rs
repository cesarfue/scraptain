use crate::error::{Result, ScraperError};
use crate::models::{Job, JobSearchParams, JobSource};
use crate::scrapers::PlatformScraper;
use async_trait::async_trait;
use reqwest::Client;
use scraper::{ElementRef, Html};
use std::time::Duration;

pub struct IndeedScraper {
    client: Client,
    base_url: String,
}

impl IndeedScraper {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            base_url: "https://www.indeed.com".to_string(),
        }
    }

    fn build_search_url(&self, params: &JobSearchParams, start: u32) -> Result<String> {
        let mut url = url::Url::parse(&format!("{}/jobs", self.base_url))?;

        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("q", &params.query);

            if let Some(location) = &params.location {
                query_pairs.append_pair("l", location);
            }

            if let Some(radius) = params.radius {
                query_pairs.append_pair("radius", &radius.to_string());
            }

            if start > 0 {
                query_pairs.append_pair("start", &start.to_string());
            }

            // Date filter
            if let Some(ref date_posted) = params.date_posted {
                let date_param = match date_posted {
                    crate::models::DatePosted::PastDay => "1",
                    crate::models::DatePosted::PastWeek => "7",
                    crate::models::DatePosted::PastMonth => "14",
                    crate::models::DatePosted::Any => "",
                };
                if !date_param.is_empty() {
                    query_pairs.append_pair("fromage", date_param);
                }
            }

            // Job type filter
            if let Some(ref job_type) = params.job_type {
                let jt = match job_type {
                    crate::models::JobType::FullTime => "fulltime",
                    crate::models::JobType::PartTime => "parttime",
                    crate::models::JobType::Contract => "contract",
                    crate::models::JobType::Temporary => "temporary",
                    crate::models::JobType::Internship => "internship",
                    _ => "",
                };
                if !jt.is_empty() {
                    query_pairs.append_pair("jt", jt);
                }
            }
        }

        Ok(url.to_string())
    }

    fn extract_job_card(&self, element: ElementRef) -> Option<Job> {
        // Extract job key/ID from the data attribute
        let job_key = IndeedScraper::extract_attr(&element, "data-jk").or_else(|| {
            // Try to get from the link href
            let link_selector = IndeedScraper::parse_selector("a.jcs-JobTitle").ok()?;
            let link = element.select(&link_selector).next()?;
            let href = IndeedScraper::extract_attr(&link, "href")?;
            href.split("jk=").nth(1)?.split('&').next()
        })?;

        // Extract title
        let title_selector = IndeedScraper::parse_selector("h2.jobTitle span").ok()?;
        let title = element
            .select(&title_selector)
            .next()
            .map(|e| IndeedScraper::extract_text(e))
            .unwrap_or_else(|| "N/A".to_string());

        // Extract company
        let company_selector =
            IndeedScraper::parse_selector("span[data-testid='company-name']").ok()?;
        let company = element
            .select(&company_selector)
            .next()
            .map(|e| IndeedScraper::extract_text(e))
            .unwrap_or_else(|| "Unknown".to_string());

        // Extract location
        let location_selector =
            IndeedScraper::parse_selector("div[data-testid='text-location']").ok()?;
        let location = element
            .select(&location_selector)
            .next()
            .map(|e| IndeedScraper::extract_text(e));

        // Extract salary if available
        let salary_selector =
            IndeedScraper::parse_selector("div[data-testid='attribute_snippet_testid']").ok()?;
        let salary = element.select(&salary_selector).find_map(|e| {
            let text = IndeedScraper::extract_text(e);
            if text.contains('$')
                || text.to_lowercase().contains("hour")
                || text.to_lowercase().contains("year")
            {
                Some(text)
            } else {
                None
            }
        });

        // Extract description snippet
        let desc_selector = IndeedScraper::parse_selector("div.job-snippet").ok()?;
        let description = element
            .select(&desc_selector)
            .next()
            .map(|e| e.inner_html().trim().to_string());

        // Extract posted date
        let date_selector =
            IndeedScraper::parse_selector("span[data-testid='myJobsStateDate']").ok()?;
        let posted_date = element
            .select(&date_selector)
            .next()
            .map(|e| IndeedScraper::extract_text(e));

        Some(Job {
            job_id: format!("in-{}", job_key),
            title,
            company,
            location,
            description,
            salary,
            url: format!("{}/viewjob?jk={}", self.base_url, job_key),
            posted_date,
            job_type: None,
            experience_level: None,
            source: JobSource::Indeed,
        })
    }
}

#[async_trait]
impl PlatformScraper for IndeedScraper {
    async fn search(&self, params: JobSearchParams) -> Result<Vec<Job>> {
        let mut all_jobs = Vec::new();
        let results_wanted = params.limit.unwrap_or(25);
        let mut start = 0u32;
        let mut page = 1;

        while all_jobs.len() < results_wanted {
            println!(
                "Indeed: search page {} / {}",
                page,
                (results_wanted as f64 / 15.0).ceil() as usize
            );

            let url = self.build_search_url(&params, start)?;

            let response = self
                .client
                .get(&url)
                .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
                .header("Accept-Language", "en-US,en;q=0.9")
                .header("Accept-Encoding", "gzip, deflate, br")
                .header("DNT", "1")
                .header("Connection", "keep-alive")
                .header("Upgrade-Insecure-Requests", "1")
                .header("Sec-Fetch-Dest", "document")
                .header("Sec-Fetch-Mode", "navigate")
                .header("Sec-Fetch-Site", "none")
                .header("Sec-Fetch-User", "?1")
                .header("Cache-Control", "max-age=0")
                .send()
                .await?;

            if !response.status().is_success() {
                if response.status() == 429 {
                    return Err(ScraperError::RateLimitExceeded);
                } else if response.status() == 403 {
                    return Err(ScraperError::BlockedByTarget);
                }
                return Err(ScraperError::RequestFailed(
                    response.error_for_status().unwrap_err(),
                ));
            }

            let html = response.text().await?;

            // Check if we're being blocked
            if html.contains("blocked") || html.contains("captcha") || html.len() < 1000 {
                return Err(ScraperError::BlockedByTarget);
            }

            // Parse and extract in a scope
            let jobs_found = {
                let document = Html::parse_document(&html);

                // Try multiple selectors as Indeed changes their HTML structure
                let job_card_selector = IndeedScraper::parse_selector("div.job_seen_beacon")
                    .or_else(|_| IndeedScraper::parse_selector("div.jobsearch-SerpJobCard"))
                    .or_else(|_| IndeedScraper::parse_selector("div[data-jk]"))
                    .or_else(|_| IndeedScraper::parse_selector("div.slider_item"))
                    .or_else(|_| IndeedScraper::parse_selector("td.resultContent"))?;

                let mut page_jobs = Vec::new();
                for element in document.select(&job_card_selector) {
                    if let Some(job) = self.extract_job_card(element) {
                        page_jobs.push(job);
                    }
                }
                page_jobs
            }; // document dropped here

            if jobs_found.is_empty() {
                break;
            }

            let found_count = jobs_found.len();
            for job in jobs_found {
                if all_jobs.len() >= results_wanted {
                    break;
                }
                all_jobs.push(job);
            }

            if all_jobs.len() >= results_wanted {
                break;
            }

            // Add delay to avoid rate limiting
            tokio::time::sleep(Duration::from_secs(3)).await;

            start += found_count as u32;
            page += 1;

            // Indeed typically shows about 15 results per page
            if start >= 1000 {
                break;
            }
        }

        all_jobs.truncate(results_wanted);

        if all_jobs.is_empty() {
            return Err(ScraperError::NoJobsFound);
        }

        Ok(all_jobs)
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
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        let scraper = IndeedScraper::new(client);

        let params = JobSearchParams {
            query: "software engineer".to_string(),
            location: Some("San Francisco".to_string()),
            limit: Some(10),
            ..Default::default()
        };

        let result = scraper.search(params).await;
        match result {
            Ok(jobs) => {
                println!("Found {} jobs", jobs.len());
                for job in jobs.iter().take(3) {
                    println!("Job: {} at {}", job.title, job.company);
                }
                assert!(!jobs.is_empty(), "Should find at least some jobs");
            }
            Err(e) => {
                println!("Error: {:?}", e);
                // Don't panic on BlockedByTarget as it's expected when scraping
                if !matches!(e, ScraperError::BlockedByTarget) {
                    panic!("Unexpected error: {:?}", e);
                }
            }
        }
    }
}
