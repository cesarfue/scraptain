use crate::constants::BoardConfig;
use crate::error::{Result, ScraperError};
use crate::models::Board;
use crate::models::{Job, JobSearchParams, PageQuery, Rule, RuleReturns};
use crate::transforms::parse_date;
use chrono::Utc;
use headless_chrome::Browser;
use scraper::{Html, Selector};
use std::time::Duration;
use tokio::time::sleep;
use url::Url;

pub struct BoardScraper {
    browser: Browser,
    config: BoardConfig,
    params: JobSearchParams,
}

impl BoardScraper {
    pub fn new() -> Result<Self> {
        let browser = Browser::default()
            .map_err(|e| ScraperError::BrowserError(format!("Failed to launch browser: {}", e)))?;

        Ok(Self {
            browser,
            config: crate::constants::HELLOWORK.clone(),
            params: JobSearchParams::default(),
        })
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

    pub fn offset(mut self, offset: u32) -> Self {
        self.params.offset = offset;
        self
    }

    pub fn single_page(mut self, single_page: bool) -> Self {
        self.params.single_page = single_page;
        self
    }

    pub fn board(mut self, board: Board) -> Self {
        self.params.board = board.clone();
        self.config = match self.params.board {
            Board::Hellowork => crate::constants::HELLOWORK.clone(),
            Board::Linkedin => crate::constants::LINKEDIN.clone(),
            Board::WTTJ => crate::constants::WTTJ.clone(),
            Board::All => crate::constants::HELLOWORK.clone(),
        };
        self
    }

    pub async fn search(self) -> Result<Vec<Job>> {
        if let Board::All = self.params.board {
            let mut futures = Vec::new();

            // Create scrapers for all boards and collect futures
            for board in Board::variants() {
                let scraper = self.create_for_board(board)?;
                futures.push(scraper.search_board());
            }

            // Run all board scrapers concurrently
            let results = futures::future::join_all(futures).await;

            // Collect all jobs from successful results
            let mut all_jobs = Vec::new();
            for result in results {
                match result {
                    Ok(jobs) => all_jobs.extend(jobs),
                    Err(e) => eprintln!("Error scraping board: {}", e),
                }
            }

            Ok(all_jobs)
        } else {
            self.search_board().await
        }
    }

    async fn search_board(self) -> Result<Vec<Job>> {
        let mut jobs = Vec::new();
        let mut count: u32 = 0;
        let mut offset: u32 = self.params.offset;
        let limit = self.params.limit;
        let single_page = self.params.single_page;

        let tab = self
            .browser
            .new_tab()
            .map_err(|e| ScraperError::BrowserError(format!("Failed to create tab: {}", e)))?;

        let mut actions_taken = false;
        while count < limit {
            let board_url = self.url(PageQuery::Board(&self.params), Some(offset))?;
            tab.navigate_to(&board_url)
                .map_err(|e| ScraperError::BrowserError(format!("Navigation failed: {}", e)))?;
            sleep(Duration::from_secs(2)).await;

            if !actions_taken {
                if let Some(action_fn) = self.config.board_page_action {
                    action_fn(&tab)?;
                }
                actions_taken = true;
            }
            let html_content = self.get_html(&tab)?;
            let document = Html::parse_document(&html_content);
            let selector =
                Selector::parse(&self.config.selectors.card.selects).expect("Invalid selector");
            let job_cards: Vec<_> = document.select(&selector).collect();
            if job_cards.is_empty() {
                eprintln!("No job cards found on page {}", offset);
                break;
            }
            for card in job_cards {
                let card_html = Html::parse_fragment(&card.html());
                let job = self.build_job(&tab, &card_html).await?;
                jobs.push(job);
                count += 1;
                if count >= limit {
                    break;
                }
            }

            // If single_page mode, stop after one page regardless of limit
            if single_page {
                break;
            }

            offset += 1;
        }
        Ok(jobs)
    }

    fn create_for_board(&self, board: Board) -> Result<Self> {
        let config = match board {
            Board::Hellowork => crate::constants::HELLOWORK.clone(),
            Board::Linkedin => crate::constants::LINKEDIN.clone(),
            Board::WTTJ => crate::constants::WTTJ.clone(),
            Board::All => panic!("Board::All should not be used here"),
        };

        let browser = Browser::default()
            .map_err(|e| ScraperError::BrowserError(format!("Failed to launch browser: {}", e)))?;

        Ok(Self {
            browser,
            config,
            params: JobSearchParams {
                board,
                ..self.params.clone()
            },
        })
    }

    async fn build_job(&self, tab: &headless_chrome::Tab, card_html: &Html) -> Result<Job> {
        let selectors = &self.config.selectors;

        let id = self
            .extract_from_rule(card_html, &selectors.id)
            .unwrap_or_default();
        let url = self.url(PageQuery::Job(&id), None)?;
        tab.navigate_to(&url)
            .map_err(|e| ScraperError::BrowserError(format!("Failed to navigate to job: {}", e)))?;

        sleep(Duration::from_secs(1)).await;

        let job_html_content = self.get_html(tab)?;
        let job_html = Html::parse_document(&job_html_content);
        let description = self
            .extract_from_rule(&job_html, &selectors.description)
            .unwrap_or_default();

        Ok(Job {
            id,
            title: self
                .extract_from_rule(card_html, &selectors.title)
                .unwrap_or_default()
                .replace('\n', " "),
            company: self
                .extract_from_rule(card_html, &selectors.company)
                .unwrap_or_default(),
            location: self
                .extract_from_rule(card_html, &selectors.location)
                .unwrap_or_default()
                .replace('\n', " "),
            description,
            url,
            date_posted: self
                .extract_from_rule(card_html, &selectors.date_posted)
                .map(|d| parse_date(&d))
                .unwrap_or_else(|| Utc::now().date_naive()),
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
                .map(|el| el.text().collect::<Vec<_>>().join("\n").trim().to_string())
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

    fn get_html(&self, tab: &headless_chrome::Tab) -> Result<String> {
        tab.get_content()
            .map_err(|e| ScraperError::BrowserError(format!("Failed to get HTML: {}", e)))
    }

    fn url(&self, query: PageQuery<'_>, offset: Option<u32>) -> Result<String> {
        match query {
            PageQuery::Board(params) => self.build_board_url(params, offset),
            PageQuery::Job(job_id) => self.build_job_url(job_id),
        }
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
