use std::time::Instant;
use crate::model::Publication::SourceId::SourceId;

#[derive(Debug)]
pub struct Provenance{
    pub source: SourceId,
    pub fetched_at: Instant,
    pub query: String,
}