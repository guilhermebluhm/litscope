use crate::model::Dblp::DblpInfo::DblpInfo;

#[derive(Default, Debug, Clone)]
#[derive(serde::Deserialize)]
pub struct DblpHitNestedContent{
    #[serde(rename="@score")]
    pub score: String,
    #[serde(rename="@id")]
    pub id: String,
    pub info: DblpInfo,
}