use crate::error::Result;
use crate::models::{Job, JobSearchParams, Selectors, UrlParameters};
use reqwest::Client;
use url::Url;

pub struct PlatformScraper {
    client: Client,
    selectors: Selectors,
    url_params: UrlParameters,
    url: &'static str,
}

impl PlatformScraper {
    async fn search(&self, params: JobSearchParams) -> Result<Vec<Job>> {
        let url = self.build_search_url(&params)?;
        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        Ok(vec![])
    }

    fn build_search_url(&self, params: &JobSearchParams) -> Result<String> {
        let url_params = self.url_params();
        let mut url = Url::parse(self.url())?;
        let mut query_pairs = url.query_pairs_mut();
        query_pairs.append_pair(&url_params.query, &params.query);
        Ok(url.to_string())
    }

    fn selectors(&self) -> &Selectors {
        &self.selectors
    }

    fn url_params(&self) -> &UrlParameters {
        &self.url_params
    }

    fn url(&self) -> &str {
        self.url
    }
}
