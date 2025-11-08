use crate::models::{SelectorRule, SelectorType, Selectors, UrlParameters};

pub struct PlatformConfig {
    pub name: &'static str,
    pub base_url: &'static str,
    pub board_path: &'static str,
    pub job_path: &'static str,
    pub selectors: Selectors,
    pub url_params: UrlParameters,
}

pub const HELLOWORK: PlatformConfig = PlatformConfig {
    name: "Hellowork",
    base_url: "https://www.hellowork.com/fr-fr/",
    board_path: "emploi/recherche.html",
    job_path: "emplois/{id}.html",
    selectors: Selectors {
        card: SelectorRule {
            selector: "li[data-id-storage-target='item']",
            n: None,
            selector_type: SelectorType::Html,
        },
        id: SelectorRule {
            selector: "li[data-id-storage-target='item']",
            n: None,
            selector_type: SelectorType::Attribute("data-id-storage-item-id"),
        },
        title: SelectorRule {
            selector: "h3.tw-inline p:first-of-type",
            n: None,
            selector_type: SelectorType::Text,
        },
        company: SelectorRule {
            selector: "h3.tw-inline p:last-of-type",
            n: None,
            selector_type: SelectorType::Text,
        },
        location: SelectorRule {
            selector: "div[data-cy='localisationCard']",
            n: None,
            selector_type: SelectorType::Text,
        },
        description: SelectorRule {
            selector: "div#offer-panel p",
            n: Some((0, 3)),
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
