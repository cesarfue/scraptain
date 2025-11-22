use crate::models::{Rule, RuleReturns, Selectors, UrlParameters};
use crate::transforms::{hellowork_date, linkedin_id};

#[derive(Clone)]
pub struct BoardConfig {
    pub name: &'static str,
    pub base_url: &'static str,
    pub board_path: &'static str,
    pub job_path: &'static str,
    pub selectors: Selectors,
    pub url_params: UrlParameters,
    pub cookie_popup_selector: Option<&'static str>,
}

pub const HELLOWORK: BoardConfig = BoardConfig {
    name: "Hellowork",
    base_url: "https://www.hellowork.com/fr-fr",
    board_path: "/emploi/recherche.html",
    job_path: "/emplois/{id}.html",
    selectors: Selectors {
        card: Rule {
            selects: "div[data-id-storage-target='item']",
            n: None,
            returns: RuleReturns::Html,
            transforms: None,
        },
        id: Rule {
            selects: "div[data-id-storage-target='item']",
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
    cookie_popup_selector: Some("button#hw-cc-notice-accept-btn"),
};

pub const LINKEDIN: BoardConfig = BoardConfig {
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
    cookie_popup_selector: None,
};
