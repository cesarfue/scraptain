use crate::models::{Rule, RuleReturns, Selectors, UrlParameters};
use crate::transforms::{hellowork_date, linkedin_id};
use once_cell::sync::Lazy;
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, UPGRADE_INSECURE_REQUESTS,
    USER_AGENT,
};

#[derive(Clone)]
pub struct BoardConfig {
    pub name: &'static str,
    pub base_url: &'static str,
    pub board_path: &'static str,
    pub job_path: &'static str,
    pub selectors: Selectors,
    pub url_params: UrlParameters,
    pub headers: Option<HeaderMap>,
}

pub const HELLOWORK: BoardConfig = BoardConfig {
    name: "Hellowork",
    base_url: "https://www.hellowork.com/fr-fr",
    board_path: "/emploi/recherche.html",
    job_path: "/emplois/{id}.html",
    selectors: Selectors {
        card: Rule {
            selects: "li[data-id-storage-target='item']",
            n: None,
            returns: RuleReturns::Html,
            transforms: None,
        },
        id: Rule {
            selects: "li[data-id-storage-target='item']",
            n: None,
            returns: RuleReturns::Attribute("data-id-storage-item-id"),
            transforms: None,
        },
        title: Rule {
            selects: "h3.tw-inline p:first-of-type",
            n: None,
            returns: RuleReturns::Text,
            transforms: None,
        },
        company: Rule {
            selects: "h3.tw-inline p:last-of-type",
            n: None,
            returns: RuleReturns::Text,
            transforms: None,
        },
        location: Rule {
            selects: "div[data-cy='localisationCard']",
            n: None,
            returns: RuleReturns::Text,
            transforms: None,
        },
        description: Rule {
            selects: "div#offer-panel p",
            n: Some((0, 3)),
            returns: RuleReturns::Text,
            transforms: None,
        },
        date_posted: Rule {
            selects: "div[class='tw-typo-s tw-text-grey-500 tw-pl-1 tw-pt-1']",
            n: None,
            returns: RuleReturns::Text,
            transforms: Some(hellowork_date),
        },
    },
    url_params: UrlParameters {
        query: "k",
        location: "l",
        offset: "p",
    },
    headers: None,
};

pub static LINKEDIN_HEADERS: Lazy<HeaderMap> = Lazy::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert("authority", HeaderValue::from_static("www.linkedin.com"));
    headers.insert(
        ACCEPT,
        HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
        ),
    );
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("max-age=0"));
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        ),
    );
    headers
});

pub static LINKEDIN: Lazy<BoardConfig> = Lazy::new(|| BoardConfig {
    name: "Linkedin",
    base_url: "https://www.linkedin.com",
    board_path: "/jobs-guest/jobs/api/seeMoreJobPostings/search?",
    job_path: "/jobs-guest/jobs/api/jobPosting/{id}",
    selectors: Selectors {
        card: Rule {
            selects: "div.base-search-card",
            n: None,
            returns: RuleReturns::Html,
            transforms: None,
        },
        id: Rule {
            selects: "div.base-search-card",
            n: None,
            returns: RuleReturns::Attribute("data-entity-urn"),
            transforms: Some(linkedin_id),
        },
        title: Rule {
            selects: "h3.base-search-card__title",
            n: None,
            returns: RuleReturns::Text,
            transforms: None,
        },
        company: Rule {
            selects: "h4.base-search-card__subtitle a",
            n: None,
            returns: RuleReturns::Text,
            transforms: None,
        },
        location: Rule {
            selects: "span.job-search-card__location",
            n: None,
            returns: RuleReturns::Text,
            transforms: None,
        },
        description: Rule {
            selects: "div.show-more-less-html__markup",
            n: None,
            returns: RuleReturns::Text,
            transforms: None,
        },
        date_posted: Rule {
            selects: "time.job-search-card__listdate",
            n: None,
            returns: RuleReturns::Attribute("datetime"),
            transforms: None,
        },
    },
    url_params: UrlParameters {
        query: "keywords",
        location: "location",
        offset: "start",
    },
    headers: Some(LINKEDIN_HEADERS.clone()),
});
