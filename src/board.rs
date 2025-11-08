use crate::constants::BoardConfig;
use crate::error::{Result, ScraperError};
use crate::models::{Job, JobSearchParams, PageQuery, Rule, RuleReturns};
use reqwest::Client;
use scraper::{Html, Selector};
use url::Url;

pub struct BoardScraper {
    client: Client,
    config: &'static BoardConfig,
}

impl BoardScraper {
    pub fn new(client: Client, config: &'static BoardConfig) -> Self {
        Self { client, config }
    }

    pub fn hellowork(client: Client) -> Self {
        Self::new(client, &crate::constants::HELLOWORK)
    }

    pub async fn search(&self, params: JobSearchParams) -> Result<Vec<Job>> {
        let mut jobs: Vec<Job> = Vec::new();
        let mut count = 0;
        let limit = params.limit.unwrap_or(100);
        let board_url = self.url(PageQuery::Board(&params))?;
        let document = self.get_html(&board_url).await?;

        while count < limit {
            let selector = Selector::parse(&self.config.selectors.card.selector).unwrap();
            let job_cards: Vec<_> = document.select(&selector).collect();
            for card in job_cards {
                if count >= limit {
                    break;
                }
                let card_html = Html::parse_fragment(&card.html());
                if let Some(job) = self.build_job(&card_html).await? {
                    jobs.push(job);
                    count += 1;
                }
            }
        }
        Ok(jobs)
    }

    async fn build_job(&self, card_html: &Html) -> Result<Option<Job>> {
        let selectors = &self.config.selectors;
        let title = self
            .extract_from_rule(&card_html, &selectors.title)
            .unwrap_or_default();
        let company = self
            .extract_from_rule(&card_html, &selectors.company)
            .unwrap_or_default();
        let location = self
            .extract_from_rule(&card_html, &selectors.location)
            .unwrap_or_default();
        let id = self
            .extract_from_rule(&card_html, &selectors.id)
            .unwrap_or_default();
        let url = self.url(PageQuery::Job(&id))?;
        let job_html = self.get_html(&url).await?;
        let description = self.extract_from_rule(&job_html, &selectors.description);

        Ok(Some(Job {
            id,
            title,
            company,
            location: Some(location),
            description,
            url,
            source: self.config.name.to_string(),
        }))
    }

    fn extract_from_rule(&self, document: &Html, selector_rule: &Rule) -> Option<String> {
        let selector = Selector::parse(selector_rule.selector).ok()?;
        let elements: Vec<_> = document.select(&selector).collect();

        if elements.is_empty() {
            return None;
        }

        let (start, end) = match selector_rule.n {
            Some((s, e)) => (s, e.min(elements.len())),
            None => (0, 1),
        };

        let slice = &elements[start..end];

        match &selector_rule.returns {
            RuleReturns::Text => {
                let texts: Vec<String> = slice
                    .iter()
                    .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                    .collect();
                Some(texts.join("\n\n"))
            }
            RuleReturns::Attribute(attr) => {
                let values: Vec<String> = slice
                    .iter()
                    .filter_map(|el| el.value().attr(attr).map(|v| v.to_string()))
                    .collect();
                Some(values.join(" "))
            }
            RuleReturns::Html => {
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
