use chrono::{Duration, Utc};

pub fn hellowork_date(text: &str) -> String {
    let text = text.trim().to_lowercase();
    let today = Utc::now().date_naive();

    if text.contains("semaine") || text.contains("week") {
        let n = text
            .split_whitespace()
            .next()
            .unwrap_or("1")
            .parse::<i64>()
            .unwrap_or(1);
        (today - Duration::weeks(n)).format("%Y-%m-%d").to_string()
    } else if text.contains("jour") || text.contains("day") {
        let n = text
            .split_whitespace()
            .next()
            .unwrap_or("1")
            .parse::<i64>()
            .unwrap_or(1);
        (today - Duration::days(n)).format("%Y-%m-%d").to_string()
    } else {
        today.format("%Y-%m-%d").to_string()
    }
}

pub fn linkedin_id(text: &str) -> String {
    text.split(':').last().unwrap_or("").to_string()
}
