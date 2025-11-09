pub struct Job {
    pub id: String,
    pub title: String,
    pub company: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub date_posted: Option<String>,
    pub url: String,
    pub source: String,
}

#[derive(Clone, Default)]
pub struct JobSearchParams {
    pub query: String,
    pub location: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

pub struct Selectors {
    pub card: Rule,
    pub id: Rule,
    pub title: Rule,
    pub company: Rule,
    pub location: Rule,
    pub description: Rule,
    pub date_posted: Rule,
}

pub enum RuleReturns {
    Text,
    Attribute(&'static str),
    Html,
}

pub enum PageQuery<'a> {
    Board(&'a JobSearchParams),
    Job(&'a str),
}

pub struct Rule {
    pub selector: &'static str,
    pub n: Option<(usize, usize)>,
    pub returns: RuleReturns,
}

pub struct UrlParameters {
    pub query: &'static str,
    pub location: &'static str,
    pub offset: &'static str,
}
