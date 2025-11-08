use crate::constants::PlatformConfig;
use crate::error::{Result, ScraperError};
use crate::models::{Job, JobSearchParams, PageQuery, SelectorRule, SelectorType};
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
        let board_url = self.url(PageQuery::Board(&params))?;
        let document = self.get_html(&board_url).await?;
        if let Some(job) = self.build_job(&document).await? {
            jobs.push(job);
        }
        Ok(jobs)
    }

    async fn build_job(&self, html: &Html) -> Result<Option<Job>> {
        let card_html = match self.extract_from_rule(html, &self.config.selectors.card) {
            Some(html_str) => Html::parse_fragment(&html_str),
            None => return Ok(None),
        };
        let title = self.extract_from_rule(&card_html, &self.config.selectors.title);
        let company = self.extract_from_rule(&card_html, &self.config.selectors.company);
        let location = self.extract_from_rule(&card_html, &self.config.selectors.location);
        let id = self.extract_from_rule(&card_html, &self.config.selectors.id);
        let url = self.url(PageQuery::Job(id.as_deref().unwrap_or_default()))?;
        let job_html = self.get_html(&url).await?;
        let description = self.extract_from_rule(&job_html, &self.config.selectors.description);

        Ok(Some(Job {
            id: id.unwrap_or_default(),
            title: title.unwrap_or_default(),
            company: company.unwrap_or_default(),
            location: Some(location.unwrap_or_default()),
            description: description,
            url: url,
            source: self.config.name.to_string(),
        }))
    }

    fn extract_from_rule(&self, document: &Html, rule: &SelectorRule) -> Option<String> {
        let selector = Selector::parse(rule.selector).ok()?;
        let elements: Vec<_> = document.select(&selector).collect();

        if elements.is_empty() {
            return None;
        }

        let (start, end) = match rule.n {
            Some((s, e)) => (s, e.min(elements.len())),
            None => (0, 1),
        };

        let slice = &elements[start..end];

        match &rule.selector_type {
            SelectorType::Text => {
                let texts: Vec<String> = slice
                    .iter()
                    .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                    .collect();
                Some(texts.join("\n\n"))
            }
            SelectorType::Attribute(attr) => {
                let values: Vec<String> = slice
                    .iter()
                    .filter_map(|el| el.value().attr(attr).map(|v| v.to_string()))
                    .collect();
                Some(values.join(" "))
            }
            SelectorType::Html => {
                let htmls: Vec<String> = slice.iter().map(|el| el.html()).collect();
                Some(htmls.join("\n"))
            }
        }
    }

    fn url(&self, query: PageQuery<'_>) -> Result<String> {
        match query {
            PageQuery::Board(params) => self.build_board_url(params),
            PageQuery::Job(job_id) => self.build_job_url(job_id),
        }
    }

    async fn get_html(&self, url: &String) -> Result<Html> {
        let response = self.client.get(url).send().await?;
        match response.status().as_u16() {
            429 => return Err(ScraperError::RateLimitExceeded),
            403 => return Err(ScraperError::BlockedByTarget),
            _ => {}
        }
        let html_text = response.text().await?;
        Ok(Html::parse_document(&html_text))
    }

    fn build_job_url(&self, job_id: &str) -> Result<String> {
        let base = Url::parse(self.config.base_url)?;
        let path = self.config.job_path.replace("{id}", job_id);
        let url = base.join(&path)?;
        Ok(url.to_string())
    }

    fn build_board_url(&self, params: &JobSearchParams) -> Result<String> {
        let mut url = Url::parse(self.config.base_url)?.join(self.config.board_path)?;
        let url_params = &self.config.url_params;

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
