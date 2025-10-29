//! # Scraptain
//!
//! A Rust library for scraping job postings from various job boards.
//!
//! ## Example
//!
//! ```no_run
//! use scraptain::{JobSearchParams, ScraperClient};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ScraperClient::new();
//!     
//!     let params = JobSearchParams {
//!         query: "rust developer".to_string(),
//!         location: Some("San Francisco".to_string()),
//!         radius: Some(25),
//!         ..Default::default()
//!     };
//!     
//!     let jobs = client.search_linkedin(params).await?;
//!     println!("Found {} jobs", jobs.len());
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod models;
pub mod scrapers;

pub use client::ScraperClient;
pub use error::{Result, ScraperError};
pub use models::{Job, JobSearchParams};
