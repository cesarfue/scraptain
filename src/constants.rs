use crate::models::{SelectorRule, SelectorType, Selectors, UrlParameters};

pub struct PlatformConfig {
    pub name: &'static str,
    pub url: &'static str,
    pub selectors: Selectors,
    pub url_params: UrlParameters,
}

pub const HELLOWORK: PlatformConfig = PlatformConfig {
    name: "Hellowork",
    url: "https://www.hellowork.com/fr-fr/emploi/recherche.html",
    selectors: Selectors {
        job_card: SelectorRule {
            selector: "li[data-id-storage-target='item']",
            nth: None,
            selector_type: SelectorType::Html,
        },
        title: SelectorRule {
            selector: "h3.tw-inline p:first-of-type",
            nth: None,
            selector_type: SelectorType::Text,
        },
        company: SelectorRule {
            selector: "h3.tw-inline p:last-of-type",
            nth: None,
            selector_type: SelectorType::Text,
        },
        location: SelectorRule {
            selector: "span.job-search-card__location",
            nth: None,
            selector_type: SelectorType::Text,
        },
        salary: SelectorRule {
            selector: "span.job-search-card__salary-info",
            nth: None,
            selector_type: SelectorType::Text,
        },
        posted_date: SelectorRule {
            selector: "time.job-search-card__listdate",
            nth: None,
            selector_type: SelectorType::Attribute("datetime"),
        },
        description: SelectorRule {
            selector: "div[class*='show-more-less-html__markup']",
            nth: None,
            selector_type: SelectorType::Text,
        },
    },
    url_params: UrlParameters {
        query: "k",
        location: "l",
        radius: "distance",
        start: "start",
        job_type: "f_JT",
        experience_level: "f_E",
        date_posted: "f_TPR",
    },
};
