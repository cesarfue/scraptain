use crate::constants::BoardConfig;
use crate::error::{Result, ScraperError};
use crate::models::Board;
use crate::models::{Job, JobSearchParams, PageQuery, Rule, RuleReturns};
use futures::future::join_all;
use reqwest::Client;
use scraper::{Html, Selector};
use std::time::Duration;
use url::Url;

pub struct BoardScraper {
    client: Client,
    config: BoardConfig,
    params: JobSearchParams,
}

impl BoardScraper {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            config: crate::constants::HELLOWORK.clone(),
            params: JobSearchParams::default(),
        }
    }

    pub fn query<S: Into<String>>(mut self, query: S) -> Self {
        self.params.query = query.into();
        self
    }

    pub fn location<S: Into<String>>(mut self, location: S) -> Self {
        self.params.location = location.into();
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.params.limit = limit;
        self
    }

    pub fn board(mut self, board: Board) -> Self {
        self.params.board = board.clone();
        self.config = match self.params.board {
            Board::Hellowork => crate::constants::HELLOWORK.clone(),
            Board::Linkedin => crate::constants::LINKEDIN.clone(),
            Board::All => crate::constants::HELLOWORK.clone(),
        };
        self
    }

    pub async fn search(self) -> Result<Vec<Job>> {
        if let Board::All = self.params.board {
            let mut futures = Vec::new();
            for board in Board::variants() {
                let scraper = self.clone_for_board(board);
                futures.push(scraper.search_board());
            }
            let results = join_all(futures).await;
            let mut all_jobs = Vec::new();
            for result in results {
                if let Ok(jobs) = result {
                    all_jobs.extend(jobs);
                }
            }
            Ok(all_jobs)
        } else {
            self.search_board().await
        }
    }

    async fn search_board(mut self) -> Result<Vec<Job>> {
        let mut jobs = Vec::new();
        let mut count: u32 = 0;
        let mut offset: u32 = 1;
        let limit = self.params.limit;

        while count < limit {
            let board_url = self.url(PageQuery::Board(&self.params), Some(offset))?;
            let document = match self.get_html(&board_url).await {
                Ok(doc) => doc,
                Err(e) => {
                    eprintln!("Failed to fetch URL {}: {:?}", board_url, e);
                    break;
                }
            };
            let selector =
                Selector::parse(&self.config.selectors.card.selects).expect("Invalid selector");
            let job_cards: Vec<_> = document.select(&selector).collect();
            if job_cards.is_empty() {
                eprintln!("No job card found");
                break;
            }
            for card in job_cards {
                let card_html = Html::parse_fragment(&card.html());
                let job = self.build_job(&card_html).await?;
                jobs.push(job);
                count += 1;
                if count >= limit {
                    break;
                }
            }
            offset += 1;
        }

        Ok(jobs)
    }

    fn clone_for_board(&self, board: Board) -> Self {
        let config = match board {
            Board::Hellowork => crate::constants::HELLOWORK.clone(),
            Board::Linkedin => crate::constants::LINKEDIN.clone(),
            Board::All => panic!("Board::All should not be used here"),
        };

        Self {
            client: self.client.clone(),
            config,
            params: JobSearchParams {
                board,
                ..self.params.clone()
            },
        }
    }

    async fn build_job(&mut self, card_html: &Html) -> Result<Job> {
        let selectors = &self.config.selectors;
        let id = self
            .extract_from_rule(card_html, &selectors.id)
            .unwrap_or_default();
        let url = self.url(PageQuery::Job(&id), None)?;
        let job_html = self.get_html(&url).await?;

        // println!("{:?}", job_html);
        let description = self
            .extract_from_rule(&job_html, &selectors.description)
            .unwrap_or_default();

        Ok(Job {
            id,
            title: self
                .extract_from_rule(card_html, &selectors.title)
                .unwrap_or_default(),
            company: self
                .extract_from_rule(card_html, &selectors.company)
                .unwrap_or_default(),
            location: self
                .extract_from_rule(card_html, &selectors.location)
                .unwrap_or_default(),
            description,
            url,
            date_posted: self
                .extract_from_rule(card_html, &selectors.date_posted)
                .unwrap_or_default(),
            source: self.config.name.to_string(),
        })
    }

    fn extract_from_rule(&self, document: &Html, selector_rule: &Rule) -> Option<String> {
        let selector = Selector::parse(selector_rule.selects).ok()?;
        let elements: Vec<_> = document.select(&selector).collect();
        if elements.is_empty() {
            return None;
        }

        let (start, end) = match selector_rule.n {
            Some((s, e)) => (s, e.min(elements.len())),
            None => (0, 1),
        };
        let slice = &elements[start..end];

        let mut values: Vec<String> = match &selector_rule.returns {
            RuleReturns::Text => slice
                .iter()
                .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .collect(),
            RuleReturns::Attribute(attr) => slice
                .iter()
                .filter_map(|el| el.value().attr(attr).map(|v| v.to_string()))
                .collect(),
            RuleReturns::Html => slice.iter().map(|el| el.html()).collect(),
        };

        if let Some(transform_fn) = selector_rule.transforms {
            values = values.iter().map(|v| transform_fn(v)).collect();
        }

        Some(values.join("\n\n"))
    }

    fn url(&self, query: PageQuery<'_>, offset: Option<u32>) -> Result<String> {
        match query {
            PageQuery::Board(params) => self.build_board_url(params, offset),
            PageQuery::Job(job_id) => self.build_job_url(job_id),
        }
    }

    async fn get_html(&self, url: &str) -> Result<Html> {
        let mut request = self.client.get(url);
        if let Some(headers) = &self.config.headers {
            request = request.headers(headers.clone());
        }
        let response = request.send().await?;
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

    fn build_board_url(&self, params: &JobSearchParams, offset: Option<u32>) -> Result<String> {
        let mut url = Url::parse(self.config.base_url)?.join(self.config.board_path)?;
        let url_params = &self.config.url_params;

        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair(url_params.query, &params.query);
            query_pairs.append_pair(url_params.location, &params.location);
            if let Some(offset) = offset {
                query_pairs.append_pair(url_params.offset, &offset.to_string());
            }
        }

        Ok(url.to_string())
    }
}
