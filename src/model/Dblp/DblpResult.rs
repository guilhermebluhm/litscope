use crate::model::Dblp::DblpHits::DblpHits;

#[derive(Default, Debug)]
#[derive(serde::Deserialize)]
pub struct DblpResult {
    /*
    modelo de arquitetura:
    hit mais interno já e o que passará a se transformar em publication
    */
    pub result: DblpHits
}