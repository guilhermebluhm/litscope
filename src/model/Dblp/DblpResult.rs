use crate::model::Dblp::DblpHits::DblpHits;

#[derive(Default, Debug)]
#[derive(serde::Deserialize)]
pub struct DblpResult {
    pub result: DblpHits
}