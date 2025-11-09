use reqwest::Client;
use scraper::Html;
use scraptain::{models::RuleReturns, BoardScraper, JobSearchParams};

fn hprint(value: Option<String>, selector_type: &RuleReturns) {
    match value {
        Some(content) => match selector_type {
            RuleReturns::Html => {
                let document = Html::parse_fragment(&content);
                println!("{:#?}", document.root_element());
            }
            RuleReturns::Text => {
                println!("{}", content);
            }
            RuleReturns::Attribute(attr) => {
                println!("{}", content);
            }
        },
        None => {
            println!("=== NO CONTENT FOUND ===\n");
        }
    }
}

#[tokio::test]
async fn test_hellowork_search() {
    let client = Client::new();
    let scraper = BoardScraper::hellowork(client);

    let params = JobSearchParams {
        query: "développeur".to_string(),
        location: Some("Lyon".to_string()),
        limit: Some(50),
        ..Default::default()
    };

    let result = scraper.search(params).await;

    match result {
        Ok(jobs) => {
            println!("Found {} jobs", jobs.len());
            for job in jobs {
                println!(
                    "title: {} | company: {} | location: {:?} | id: {} | url: {:?}",
                    job.title, job.company, job.location, job.id, job.url
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
