use crate::models::{Rule, RuleReturns, Selectors, UrlParameters};

pub struct BoardConfig {
    pub name: &'static str,
    pub base_url: &'static str,
    pub board_path: &'static str,
    pub job_path: &'static str,
    pub selectors: Selectors,
    pub url_params: UrlParameters,
}

pub const HELLOWORK: BoardConfig = BoardConfig {
    name: "Hellowork",
    base_url: "https://www.hellowork.com/fr-fr/",
    board_path: "emploi/recherche.html",
    job_path: "emplois/{id}.html",
    selectors: Selectors {
        card: Rule {
            selector: "li[data-id-storage-target='item']",
            n: None,
            returns: RuleReturns::Html,
        },
        id: Rule {
            selector: "li[data-id-storage-target='item']",
            n: None,
            returns: RuleReturns::Attribute("data-id-storage-item-id"),
        },
        title: Rule {
            selector: "h3.tw-inline p:first-of-type",
            n: None,
            returns: RuleReturns::Text,
        },
        company: Rule {
            selector: "h3.tw-inline p:last-of-type",
            n: None,
            returns: RuleReturns::Text,
        },
        location: Rule {
            selector: "div[data-cy='localisationCard']",
            n: None,
            returns: RuleReturns::Text,
        },
        description: Rule {
            selector: "div#offer-panel p",
            n: Some((0, 3)),
            returns: RuleReturns::Text,
        },
    },
    url_params: UrlParameters {
        query: "k",
        location: "l",
        radius: "distance",
        offset: "p",
        job_type: "f_JT",
        experience_level: "f_E",
        date_posted: "f_TPR",
    },
};
