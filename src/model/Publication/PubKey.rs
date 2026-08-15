use crate::model::Publication::Doi::Doi;

#[derive(Debug, Default)]
pub enum PubKey {
    #[default]
    DefaultCase,
    Doi(Doi),
    ArxivId(String),
    Fingerprint(String),
}