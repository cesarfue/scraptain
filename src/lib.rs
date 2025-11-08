pub mod client;
pub mod constants;
pub mod error;
pub mod models;
pub mod platform;

pub use client::ScraperClient;
pub use error::{Result, ScraperError};
pub use models::{Job, JobSearchParams};
pub use platform::PlatformScraper;
