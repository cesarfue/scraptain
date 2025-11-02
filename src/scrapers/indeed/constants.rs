use crate::models::Selectors;

pub const SELECTORS: Selectors = Selectors {
    job_card: "div.job_seen_beacon",
    title: "h2.jobTitle span",
    company: "span[data-testid='company-name']",
    location: "div[data-testid='text-location']",
    salary: "div[data-testid='attribute_snippet_testid']",
    posted_date: "span[data-testid='myJobsStateDate']",
    description: "div.job-snippet",
};

pub const FALLBACK_SELECTORS: &[&str] = &[
    "div.jobsearch-SerpJobCard",
    "div[data-jk]",
    "div.slider_item",
    "td.resultContent",
];

pub const BASE_URL: &str = "https://www.indeed.com";
pub const DEFAULT_DELAY: u64 = 3;

fn job_type_code(job_type: &crate::models::JobType) -> Option<&'static str> {
    use crate::models::JobType::*;
    match job_type {
        FullTime => Some("fulltime"),
        PartTime => Some("parttime"),
        Contract => Some("contract"),
        Temporary => Some("temporary"),
        Internship => Some("internship"),
        _ => None,
    }
}

fn indeed_days(date: &crate::models::DatePosted) -> Option<&'static str> {
    use crate::models::DatePosted::*;
    match date {
        PastDay => Some("1"),
        PastWeek => Some("7"),
        PastMonth => Some("14"),
        Any => None,
    }
}
