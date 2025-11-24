use scraptain::{Board, BoardScraper};

#[tokio::test]
async fn test_hellowork() {
    let result = BoardScraper::new()
        .expect("Failed to create scraper")
        .query("développeur")
        .location("Lyon, France")
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
                    "\n{}\n  Company: {}\n  Location: {}\n  Source: {}\n  URL: {}\n  Date posted: {:?}\n  Description: {}",
                    job.title, job.company, job.location, job.source, job.url, job.date_posted, job.description
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
async fn test_linkedin() {
    let result = BoardScraper::new()
        .expect("Failed to create scraper")
        .query("développeur")
        .location("Lyon, France")
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
                    "\n{}\n  Company: {}\n  Location: {}\n  Source: {}\n  URL: {}\n  Date posted: {:?}\n  Description: {}",
                    job.title, job.company, job.location, job.source, job.url, job.date_posted, job.description
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
async fn test_wttj() {
    let result = BoardScraper::new()
        .expect("Failed to create scraper")
        .query("développeur")
        .location("Lyon, France")
        .limit(5)
        .board(Board::WTTJ)
        .search()
        .await;

    match result {
        Ok(jobs) => {
            println!("Found {} jobs from WTTJ", jobs.len());
            assert!(!jobs.is_empty(), "Should find at least one job");

            for job in jobs {
                println!(
                    "\n{}\n  Company: {}\n  Location: {}\n  Source: {}\n  URL: {}\n  Date posted: {:?}\n  Description: {}",
                    job.title, job.company, job.location, job.source, job.url, job.date_posted, job.description
                );

                assert_eq!(job.source, "WelcomeToTheJungle");
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
async fn test_all() {
    let result = BoardScraper::new()
        .expect("Failed to create scraper")
        .query("développeur")
        .location("Lyon, France")
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
            let mut wttj_count = 0;

            for job in jobs {
                println!(
                    "{}\n  Company: {}\n  Location: {}\n  Source: {}\n  URL: {}",
                    job.title, job.company, job.location, job.source, job.url
                );

                match job.source.as_str() {
                    "Hellowork" => hellowork_count += 1,
                    "Linkedin" => linkedin_count += 1,
                    "WelcomeToTheJungle" => wttj_count += 1,
                    _ => {}
                }

                assert!(!job.id.is_empty());
                assert!(!job.title.is_empty());
                assert!(!job.company.is_empty());
            }

            println!(
                "\nResults breakdown: {} Hellowork, {} LinkedIn, {} WTTJ",
                hellowork_count, linkedin_count, wttj_count
            );
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Test failed: {}", e);
        }
    }
}
