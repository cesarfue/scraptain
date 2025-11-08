use reqwest::Client;
use scraper::Html;
use scraptain::{models::SelectorType, JobSearchParams, PlatformScraper};

fn hprint(value: Option<String>, selector_type: &SelectorType) {
    match value {
        Some(content) => match selector_type {
            SelectorType::Html => {
                let document = Html::parse_fragment(&content);
                println!("{:#?}", document.root_element());
            }
            SelectorType::Text => {
                println!("{}", content);
            }
            SelectorType::Attribute(attr) => {
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
    let scraper = PlatformScraper::hellowork(client);

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
                    "title: {} | company: {} | location: {:?} | id: {} | url: {:?}\n description: {:?}",
                    job.title, job.company, job.location, job.id, job.url, job.description
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
