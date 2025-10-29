use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
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
    pub job_id: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobSource {
    LinkedIn,
    Indeed,
}

impl std::fmt::Display for JobSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobSource::LinkedIn => write!(f, "LinkedIn"),
            JobSource::Indeed => write!(f, "Indeed"),
        }
    }
}
