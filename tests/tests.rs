use scraptain::{Board, BoardScraper};

#[tokio::test]
async fn hellowork() {
    let result = BoardScraper::new()
        .query("développeur")
        .location("Lyon")
        .limit(5)
        .board(Board::Hellowork)
        .search()
        .await;

    match result {
        Ok(jobs) => {
            println!("Found {} jobs from Hellowork", jobs.len());
            assert!(!jobs.is_empty(), "Should find at least one job");

            for job in jobs {
                println!(
                    "\n{}\n{}\n  Company: {}\n  Location: {}\n  Source: {}\n  URL: {}\n Date posted: {}\n Description: {}",
                    job.id, job.title, job.company, job.location, job.source, job.url, job.date_posted, job.description
                );

                assert_eq!(job.source, "Hellowork");
                assert!(!job.id.is_empty());
                assert!(!job.title.is_empty());
                assert!(!job.company.is_empty());
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Test failed: {}", e);
        }
    }
}

#[tokio::test]
async fn linkedin() {
    let result = BoardScraper::new()
        .query("développeur")
        .location("Lyon")
        .limit(5)
        .board(Board::Linkedin)
        .search()
        .await;

    match result {
        Ok(jobs) => {
            println!("Found {} jobs from LinkedIn", jobs.len());
            assert!(!jobs.is_empty(), "Should find at least one job");

            for job in jobs {
                println!(
                    "\n{}\n{}\n  Company: {}\n  Location: {}\n  Source: {}\n  URL: {}\n Date posted: {}\n Description: {}",
                    job.id, job.title, job.company, job.location, job.source, job.url, job.date_posted, job.description
                );

                assert_eq!(job.source, "Linkedin");
                assert!(!job.id.is_empty());
                assert!(!job.title.is_empty());
                assert!(!job.company.is_empty());
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Test failed: {}", e);
        }
    }
}

#[tokio::test]
async fn all() {
    let result = BoardScraper::new()
        .query("développeur")
        .location("Lyon")
        .limit(3)
        .board(Board::All)
        .search()
        .await;

    match result {
        Ok(jobs) => {
            println!("Found {} jobs from all boards", jobs.len());
            assert!(!jobs.is_empty(), "Should find at least one job");

            let mut hellowork_count = 0;
            let mut linkedin_count = 0;

            for job in jobs {
                println!(
                    "\n{}\n  Company: {}\n  Location: {}\n  Source: {}\n  URL: {}",
                    job.title, job.company, job.location, job.source, job.url
                );

                match job.source.as_str() {
                    "Hellowork" => hellowork_count += 1,
                    "Linkedin" => linkedin_count += 1,
                    _ => {}
                }

                assert!(!job.id.is_empty());
                assert!(!job.title.is_empty());
                assert!(!job.company.is_empty());
            }

            println!(
                "\nResults breakdown: {} Hellowork, {} LinkedIn",
                hellowork_count, linkedin_count
            );
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Test failed: {}", e);
        }
    }
}
