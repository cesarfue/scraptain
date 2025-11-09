pub mod board;
pub mod constants;
pub mod error;
pub mod models;
pub mod transforms;

pub use board::BoardScraper;
pub use error::{Result, ScraperError};
pub use models::{Board, Job, JobSearchParams};
