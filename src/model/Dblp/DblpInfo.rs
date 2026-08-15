use crate::model::Dblp::DblpInfoAuthor::DblpInfoAuthor;

#[derive(Default, Debug, Clone)]
#[derive(serde::Deserialize)]
pub struct DblpInfo {
    pub key: String,
    pub authors: DblpInfoAuthor,
    pub title: String,
    pub venue: String,
    pub year: String,
    pub doi: String,
    #[serde(rename="type")]
    pub type_pub_info: String
}