use crate::models::{Selectors, UrlParameters};

pub const SELECTORS: Selectors = Selectors {
    job_card: "div.base-search-card",
    title: "span.sr-only",
    company: "h4.base-search-card__subtitle",
    location: "span.job-search-card__location",
    salary: "span.job-search-card__salary-info",
    posted_date: "time.job-search-card__listdate",
    description: "div[class*='show-more-less-html__markup']",
};

pub const URL_PARAMS: UrlParameters = UrlParameters {
    query: "keywords",
    location: "location",
    radius: "distance",
    start: "start",
    job_type: "f_JT",
    experience_level: "f_E",
    date_posted: "f_TPR",
};

pub const BASE_URL: &str = "https://www.linkedin.com";
pub const SEARCH_ENDPOINT: &str = "/jobs-guest/jobs/api/seeMoreJobPostings/search";
pub const DEFAULT_DELAY: u64 = 3;
pub const DELAY_VARIANCE: u64 = 4;
