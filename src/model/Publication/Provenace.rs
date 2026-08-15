use crate::model::Publication::SourceId::SourceId;

#[derive(Debug)]
pub struct Provenance{
    pub source: SourceId,
    pub native_id: String,
    pub fetched_at: i64,
    pub query: String,
}