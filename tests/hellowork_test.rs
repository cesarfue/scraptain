use reqwest::Client;
use scraptain::{BoardScraper, JobSearchParams};

#[tokio::test]
async fn test_hellowork_search() {
    let client = Client::new();
    let scraper = BoardScraper::hellowork(client);

    let params = JobSearchParams {
        query: "développeur".to_string(),
        location: Some("Lyon".to_string()),
        limit: Some(5),
        ..Default::default()
    };

    let result = scraper.search(params).await;

    match result {
        Ok(jobs) => {
            println!("Found {} jobs", jobs.len());
            for job in jobs {
                println!(
                    "title: {} | company: {} | location: {:?} | id: {} | url: {:?} | date_posted: {:?}",
                    job.title, job.company, job.location, job.id, job.url, job.date_posted
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
