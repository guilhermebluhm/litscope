use crate::model::Dblp::DblpHitNestedContent::DblpHitNestedContent;
use crate::wrapper::HitWrapper::HitWrapper;

#[derive(Default, Debug)]
#[derive(serde::Deserialize)]
pub struct DblpHitContent{

    #[serde(rename="@total")]
    pub total: String,
    #[serde(rename="@sent")]
    pub sent: String,
    #[serde(rename="@first")]
    pub first: String,
    pub hit: Option<HitWrapper>
}