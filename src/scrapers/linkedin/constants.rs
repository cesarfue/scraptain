use crate::models::Selectors;

pub const SELECTORS: Selectors = Selectors {
    job_card: "div.base-search-card",
    title: "span.sr-only",
    company: "h4.base-search-card__subtitle",
    location: "span.job-search-card__location",
    salary: "span.job-search-card__salary-info",
    posted_date: "time.job-search-card__listdate",
    description: "div[class*='show-more-less-html__markup']",
};

pub const BASE_URL: &str = "https://www.linkedin.com";
pub const SEARCH_ENDPOINT: &str = "/jobs-guest/jobs/api/seeMoreJobPostings/search";
pub const DEFAULT_DELAY: u64 = 3;
pub const DELAY_VARIANCE: u64 = 4;

pub fn job_type_code(job_type: &crate::models::JobType) -> Option<&'static str> {
    use crate::models::JobType::*;
    match job_type {
        FullTime => Some("F"),
        PartTime => Some("P"),
        Contract => Some("C"),
        Temporary => Some("T"),
        Internship => Some("I"),
        Volunteer => Some("V"),
    }
}

pub fn experience_level_code(level: &crate::models::ExperienceLevel) -> Option<&'static str> {
    use crate::models::ExperienceLevel::*;
    match level {
        Internship => Some("1"),
        EntryLevel => Some("2"),
        Associate => Some("3"),
        MidSenior => Some("4"),
        Director => Some("5"),
        Executive => Some("6"),
    }
}

pub fn date_posted_seconds(date: &crate::models::DatePosted) -> Option<u32> {
    use crate::models::DatePosted::*;
    match date {
        PastDay => Some(86400),
        PastWeek => Some(604800),
        PastMonth => Some(2592000),
        Any => None,
    }
}
