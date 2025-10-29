pub mod indeed;
pub mod linkedin;

use crate::error::Result;
use crate::models::{Job, JobSearchParams};
use async_trait::async_trait;

#[async_trait]
pub trait JobScraper: Send + Sync {
    async fn search(&self, params: JobSearchParams) -> Result<Vec<Job>>;

    fn name(&self) -> &str;
}
