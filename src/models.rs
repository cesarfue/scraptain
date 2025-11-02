use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobSource {
    LinkedIn,
    Indeed,
}

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
    pub source: JobSource,
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

impl std::fmt::Display for JobSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobSource::LinkedIn => write!(f, "LinkedIn"),
            JobSource::Indeed => write!(f, "Indeed"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Selectors {
    pub job_card: &'static str,
    pub title: &'static str,
    pub company: &'static str,
    pub location: &'static str,
    pub salary: &'static str,
    pub posted_date: &'static str,
    pub description: &'static str,
}
