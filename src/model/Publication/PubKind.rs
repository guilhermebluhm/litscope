#[derive(Debug, Default)]
pub enum PubKind{
    #[default]
    DefaultCase,
    Conference,
    Journal,
    Preprint,
    Bookchapter,
    Thesis,
    Other
}