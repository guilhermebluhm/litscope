#[derive(Debug, Default)]
pub struct Author{
    pub fullName: String,
    pub family: String,
    pub given: Option<String>,
    pub sourceId: Option<String>, //pid
}