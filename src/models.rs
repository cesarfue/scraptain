use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub job_id: String,
    pub title: String,
    pub company: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub salary: Option<String>,
    pub url: String,
    pub posted_date: Option<String>,
    pub job_type: Option<String>,
    pub experience_level: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Default)]
pub struct JobSearchParams {
    pub query: String,
    pub location: Option<String>,
    pub radius: Option<u32>,
    pub job_type: Option<JobType>,
    pub experience_level: Option<ExperienceLevel>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub date_posted: Option<DatePosted>,
}

pub struct Selectors {
    pub job_card: SelectorRule,
    pub title: SelectorRule,
    pub company: SelectorRule,
    pub location: SelectorRule,
    pub salary: SelectorRule,
    pub posted_date: SelectorRule,
    pub description: SelectorRule,
}

#[derive(Debug)]
pub enum SelectorType {
    Text,
    Attribute(&'static str),
    Html,
}

#[derive(Debug)]
pub struct SelectorRule {
    pub selector: &'static str,
    pub nth: Option<usize>,
    pub selector_type: SelectorType,
}

#[derive(Debug, Clone)]
pub struct UrlParameters {
    pub query: &'static str,
    pub location: &'static str,
    pub radius: &'static str,
    pub start: &'static str,
    pub job_type: &'static str,
    pub experience_level: &'static str,
    pub date_posted: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobType {
    FullTime,
    PartTime,
    Contract,
    Temporary,
    Internship,
    Volunteer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperienceLevel {
    Internship,
    EntryLevel,
    Associate,
    MidSenior,
    Director,
    Executive,
}

#[derive(Debug, Clone)]
pub enum DatePosted {
    PastDay,
    PastWeek,
    PastMonth,
    Any,
}

impl DatePosted {
    pub fn to_hours(&self) -> Option<u32> {
        match self {
            DatePosted::PastDay => Some(24),
            DatePosted::PastWeek => Some(168),
            DatePosted::PastMonth => Some(720),
            DatePosted::Any => None,
        }
    }

    pub fn to_seconds(&self) -> Option<u32> {
        match self {
            DatePosted::PastDay => Some(86400),
            DatePosted::PastWeek => Some(604800),
            DatePosted::PastMonth => Some(2592000),
            DatePosted::Any => None,
        }
    }
}
