use thiserror::Error;

pub type Result<T> = std::result::Result<T, ScraperError>;

#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Browser error: {0}")]
    BrowserError(String),

    #[error("Element not found")]
    ElementNotFound,

    #[error("Extraction failed: {0}")]
    ExtractionFailed(String),
}
