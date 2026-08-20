#[derive(Debug, Default)]
pub struct Query {
    pub terms: Vec<String>,
    pub phrase: Option<String>,
    pub year_range: Option<(u16, u16)>,
    pub name: String,
    pub type_source: Vec<String>
}