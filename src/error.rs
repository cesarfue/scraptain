use thiserror::Error;

pub type Result<T> = std::result::Result<T, ScraperError>;

#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse HTML: {0}")]
    ParseError(String),

    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Invalid search parameters")]
    InvalidParameters,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Scraper blocked or captcha detected")]
    BlockedByTarget,

    #[error("No jobs found")]
    NoJobsFound,

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
