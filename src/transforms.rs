use chrono::{Duration, NaiveDate, NaiveDateTime, Utc};

pub fn hellowork_date(text: &str) -> String {
    let text = text.trim().to_lowercase();
    let today = Utc::now().date_naive();

    let date = if text.contains("semaine") || text.contains("week") {
        let n = extract_number(&text);
        today - Duration::weeks(n)
    } else if text.contains("jour") || text.contains("day") {
        let n = extract_number(&text);
        today - Duration::days(n)
    } else if text.contains("mois") || text.contains("month") {
        let n = extract_number(&text);
        today - Duration::days(n * 30)
    } else {
        today
    };

    date.to_string()
}

fn extract_number(text: &str) -> i64 {
    // Find the first number in the text
    text.split_whitespace()
        .find_map(|word| word.parse::<i64>().ok())
        .unwrap_or(1)
}

pub fn linkedin_id(text: &str) -> String {
    text.split(':').last().unwrap_or("").to_string()
}

pub fn parse_date(text: &str) -> NaiveDate {
    if text.contains('T') {
        NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M:%SZ")
            .map(|dt| dt.date())
            .unwrap_or_else(|_| Utc::now().date_naive())
    } else {
        NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap_or_else(|_| Utc::now().date_naive())
    }
}
