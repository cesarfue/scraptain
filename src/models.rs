pub struct Job {
    pub id: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub description: String,
    pub date_posted: String,
    pub url: String,
    pub source: String,
}

#[derive(Clone)]
pub enum Board {
    Hellowork,
    Linkedin,
    All,
}

impl Board {
    pub fn variants() -> Vec<Board> {
        vec![Board::Hellowork, Board::Linkedin]
    }
}

#[derive(Clone)]
pub struct JobSearchParams {
    pub query: String,
    pub board: Board,
    pub location: String,
    pub limit: u32,
}

impl Default for JobSearchParams {
    fn default() -> Self {
        Self {
            query: String::new(),
            board: Board::All,
            location: String::new(),
            limit: 50,
        }
    }
}

#[derive(Clone)]
pub struct Selectors {
    pub card: Rule,
    pub id: Rule,
    pub title: Rule,
    pub company: Rule,
    pub location: Rule,
    pub description: Rule,
    pub date_posted: Rule,
}

#[derive(Clone)]
pub enum RuleReturns {
    Text,
    Attribute(&'static str),
    Html,
}

#[derive(Clone)]
pub struct Rule {
    pub selects: &'static str,
    pub n: Option<(usize, usize)>,
    pub returns: RuleReturns,
    pub transforms: Option<fn(&str) -> String>,
}

pub enum PageQuery<'a> {
    Board(&'a JobSearchParams),
    Job(&'a str),
}

#[derive(Clone)]
pub struct UrlParameters {
    pub query: &'static str,
    pub location: &'static str,
    pub offset: &'static str,
}
