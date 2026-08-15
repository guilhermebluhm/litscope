use crate::model::Dblp::DblpHitContent::DblpHitContent;

#[derive(Default, Debug)]
#[derive(serde::Deserialize)]
pub struct DblpHits {
    pub hits: DblpHitContent
}