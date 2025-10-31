use crate::{
    error::{Result, ScraperError},
    models::{Job, JobSearchParams, JobSource},
    scrapers::PlatformScraper,
};
use async_trait::async_trait;
use chrono::NaiveDate;
use rand::Rng;
use reqwest::Client;
use scraper::{ElementRef, Html};
use std::time::Duration;

pub struct LinkedInScraper {
    client: Client,
    base_url: String,
    delay: u64,
    band_delay: u64,
}

#[derive(Debug)]
struct JobCardData {
    job_id: String,
    title: String,
    company: String,
    location: Option<String>,
    salary: Option<String>,
    posted_date: Option<String>,
}

impl LinkedInScraper {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            base_url: "https://www.linkedin.com".to_string(),
            delay: 3,
            band_delay: 4,
        }
    }

    fn build_search_url(&self, params: &JobSearchParams, start: u32) -> Result<String> {
        let base_url = format!(
            "{}/jobs-guest/jobs/api/seeMoreJobPostings/search",
            self.base_url
        );
        let mut url = url::Url::parse(&base_url)?;

        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("keywords", &params.query);

            if let Some(location) = &params.location {
                query_pairs.append_pair("location", location);
            }

            if let Some(radius) = params.radius {
                query_pairs.append_pair("distance", &radius.to_string());
            }

            if let Some(ref job_type) = params.job_type {
                if let Some(code) = job_type.linkedin_code() {
                    query_pairs.append_pair("f_JT", code);
                }
            }

            if let Some(ref experience_level) = params.experience_level {
                if let Some(code) = experience_level.linkedin_code() {
                    query_pairs.append_pair("f_E", code);
                }
            }

            query_pairs.append_pair("pageNum", "0");
            query_pairs.append_pair("start", &start.to_string());

            if let Some(ref date_posted) = params.date_posted {
                if let Some(seconds) = date_posted.to_seconds() {
                    query_pairs.append_pair("f_TPR", &format!("r{}", seconds));
                }
            }
        }

        Ok(url.to_string())
    }

    fn extract_job_card_data(&self, element: ElementRef) -> Option<JobCardData> {
        let link_selector = LinkedInScraper::parse_selector("a.base-card__full-link").ok()?;
        let link_element = element.select(&link_selector).next()?;
        let href = LinkedInScraper::extract_attr(&link_element, "href")?;
        let job_id = href.split('-').last()?.split('?').next()?.to_string();

        let title_selector = LinkedInScraper::parse_selector("span.sr-only").ok()?;
        let title = element
            .select(&title_selector)
            .next()
            .map(|e| LinkedInScraper::extract_text(e))
            .unwrap_or_else(|| "N/A".to_string());

        let company = self.extract_company(element)?;
        let location = self.extract_location(element);
        let salary = self.extract_salary(element);
        let posted_date = self.extract_posted_date(element);

        Some(JobCardData {
            job_id,
            title,
            company,
            location,
            salary,
            posted_date,
        })
    }

    async fn build_job_from_card_data(
        &self,
        card_data: JobCardData,
        fetch_description: bool,
    ) -> Job {
        let job_url = format!("{}/jobs/view/{}", self.base_url, card_data.job_id);

        let (description, job_type, experience_level) = if fetch_description {
            self.fetch_job_details(&card_data.job_id)
                .await
                .unwrap_or((None, None, None))
        } else {
            (None, None, None)
        };

        Job {
            job_id: format!("li-{}", card_data.job_id),
            title: card_data.title,
            company: card_data.company,
            location: card_data.location,
            description,
            salary: card_data.salary,
            url: job_url,
            posted_date: card_data.posted_date,
            job_type,
            experience_level,
            source: JobSource::LinkedIn,
        }
    }

    fn extract_company(&self, element: ElementRef) -> Option<String> {
        let company_selector =
            LinkedInScraper::parse_selector("h4.base-search-card__subtitle").ok()?;
        let company_element = element.select(&company_selector).next()?;
        let company_link_selector = LinkedInScraper::parse_selector("a").ok()?;

        Some(
            company_element
                .select(&company_link_selector)
                .next()
                .map(|e| LinkedInScraper::extract_text(e))
                .unwrap_or_else(|| "N/A".to_string()),
        )
    }

    fn extract_location(&self, element: ElementRef) -> Option<String> {
        let location_selector =
            LinkedInScraper::parse_selector("span.job-search-card__location").ok()?;
        element
            .select(&location_selector)
            .next()
            .map(|e| LinkedInScraper::extract_text(e))
    }

    fn extract_salary(&self, element: ElementRef) -> Option<String> {
        let salary_selector =
            LinkedInScraper::parse_selector("span.job-search-card__salary-info").ok()?;
        element
            .select(&salary_selector)
            .next()
            .map(|e| LinkedInScraper::extract_text(e))
    }

    fn extract_posted_date(&self, element: ElementRef) -> Option<String> {
        let date_selector =
            LinkedInScraper::parse_selector("time.job-search-card__listdate").ok()?;
        element
            .select(&date_selector)
            .next()
            .and_then(|e| LinkedInScraper::extract_attr(&e, "datetime"))
            .and_then(|date_str| NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok())
            .map(|date_str| date_str.to_string())
    }

    async fn fetch_job_details(
        &self,
        job_id: &str,
    ) -> Option<(Option<String>, Option<String>, Option<String>)> {
        let job_url = format!("{}/jobs/view/{}", self.base_url, job_id);

        let response = self.client.get(&job_url).send().await.ok()?;

        if !response.status().is_success() {
            return None;
        }

        let html = response.text().await.ok()?;

        if html.contains("linkedin.com/signup") {
            return None;
        }

        let document = Html::parse_document(&html);
        let description = self.parse_description(&document);
        let job_type = self.parse_job_type_from_html(&document);
        let experience_level = self.parse_experience_level(&document);

        Some((description, job_type, experience_level))
    }

    fn parse_description(&self, document: &Html) -> Option<String> {
        let desc_selector =
            LinkedInScraper::parse_selector("div[class*='show-more-less-html__markup']").ok()?;
        document
            .select(&desc_selector)
            .next()
            .map(|e| e.inner_html().trim().to_string())
    }

    fn parse_job_type_from_html(&self, document: &Html) -> Option<String> {
        self.parse_job_criteria(document, "Employment type")
    }

    fn parse_experience_level(&self, document: &Html) -> Option<String> {
        self.parse_job_criteria(document, "Seniority level")
    }

    fn parse_job_criteria(&self, document: &Html, criteria_name: &str) -> Option<String> {
        let h3_selector = LinkedInScraper::parse_selector("h3").ok()?;
        let span_selector =
            LinkedInScraper::parse_selector("span.description__job-criteria-text").ok()?;

        for h3 in document.select(&h3_selector) {
            let text = h3.text().collect::<String>();
            if text.contains(criteria_name) {
                if let Some(parent_node) = h3.parent() {
                    if let Some(parent_elem) = ElementRef::wrap(parent_node) {
                        for span_elem in parent_elem.select(&span_selector) {
                            return Some(LinkedInScraper::extract_text(span_elem));
                        }
                    }
                }
            }
        }
        None
    }

    async fn random_delay(&self) {
        let delay = {
            let mut rng = rand::thread_rng();
            rng.gen_range(self.delay..=(self.delay + self.band_delay))
        };

        tokio::time::sleep(Duration::from_secs(delay)).await;
    }
}

#[async_trait]
impl PlatformScraper for LinkedInScraper {
    async fn search(&self, params: JobSearchParams) -> Result<Vec<Job>> {
        let mut all_jobs = Vec::new();
        let results_wanted = params.limit.unwrap_or(25);
        let mut start = params.offset.unwrap_or(0) as u32;
        let max_start = 1000u32;
        let fetch_description = true;
        let mut request_count = 0;

        while all_jobs.len() < results_wanted && start < max_start {
            request_count += 1;
            println!(
                "LinkedIn: search page {} / {}",
                request_count,
                (results_wanted as f64 / 25.0).ceil() as usize
            );

            let url = self.build_search_url(&params, start)?;
            let response = self.client.get(&url).send().await?;

            if !response.status().is_success() {
                if response.status() == 429 {
                    return Err(ScraperError::RateLimitExceeded);
                }
                return Err(ScraperError::RequestFailed(
                    response.error_for_status().unwrap_err(),
                ));
            }

            let html = response.text().await?;

            let (card_data_list, jobs_count) = {
                let document = Html::parse_document(&html);
                let job_card_selector = LinkedInScraper::parse_selector("div.base-search-card")?;

                let mut card_data_list = Vec::new();
                for element in document.select(&job_card_selector) {
                    if let Some(card_data) = self.extract_job_card_data(element) {
                        card_data_list.push(card_data);
                    }
                }

                let jobs_count = card_data_list.len();
                (card_data_list, jobs_count)
            };

            if card_data_list.is_empty() {
                break;
            }

            for card_data in card_data_list {
                if all_jobs.len() >= results_wanted {
                    break;
                }

                let job = self
                    .build_job_from_card_data(card_data, fetch_description)
                    .await;
                all_jobs.push(job);
            }

            if all_jobs.len() >= results_wanted {
                break;
            }

            self.random_delay().await;
            start += jobs_count as u32;
        }

        all_jobs.truncate(results_wanted);
        Ok(all_jobs)
    }

    fn name(&self) -> &str {
        "LinkedIn"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{DatePosted, ExperienceLevel, JobType};

    #[tokio::test]
    async fn test_linkedin_scraper() {
        let client = Client::new();
        let scraper = LinkedInScraper::new(client);

        let params = JobSearchParams {
            query: "rust developer".to_string(),
            location: Some("San Francisco".to_string()),
            limit: Some(10),
            job_type: Some(JobType::FullTime),
            experience_level: Some(ExperienceLevel::MidSenior),
            date_posted: Some(DatePosted::PastWeek),
            ..Default::default()
        };

        let result = scraper.search(params).await;
        match result {
            Ok(jobs) => {
                println!("Found {} jobs", jobs.len());
                for job in jobs.iter() {
                    println!("Job: {} at {}", job.title, job.company);
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
