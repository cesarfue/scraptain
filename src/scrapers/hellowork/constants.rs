use crate::models::{Selectors, UrlParameters};

pub const SELECTORS: Selectors = Selectors {
    job_card: "li[data-id-storage-target='item']",
    title: "h3.tw-inline p:first-of-type",
    company: "h3.tw-inline p:last-of-type",
    location: "span.job-search-card__location",
    salary: "span.job-search-card__salary-info",
    posted_date: "time.job-search-card__listdate",
    description: "div[class*='show-more-less-html__markup']",
};

pub const URL_PARAMS: UrlParameters = UrlParameters {
    query: "k",
    location: "l",
    radius: "distance",
    start: "start",
    job_type: "f_JT",
    experience_level: "f_E",
    date_posted: "f_TPR",
};

pub const BASE_URL: &str = "https://www.hellowork.com";
pub const SEARCH_ENDPOINT: &str = "/fr-fr/emploi/recherche.html?";
pub const DEFAULT_DELAY: u64 = 3;
pub const DELAY_VARIANCE: u64 = 4;
