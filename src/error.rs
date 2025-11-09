use thiserror::Error;

pub type Result<T> = std::result::Result<T, ScraperError>;

#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Scraper blocked or captcha detected")]
    BlockedByTarget,

    #[error("WebDriver error: {0}")]
    WebDriverError(String),
}
