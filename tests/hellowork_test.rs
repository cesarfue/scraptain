use reqwest::Client;
use scraptain::{JobSearchParams, PlatformScraper};

#[tokio::test]
#[ignore]
async fn test_hellowork_search() {
    let client = Client::new();
    let scraper = PlatformScraper::hellowork(client);

    let params = JobSearchParams {
        query: "développeur".to_string(),
        location: Some("Paris".to_string()),
        limit: Some(5),
        ..Default::default()
    };

    let result = scraper.search(params).await;

    match result {
        Ok(jobs) => {
            println!("Found {} jobs", jobs.len());
            for job in jobs {
                println!("{} at {} - {}", job.title, job.company, job.url);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
