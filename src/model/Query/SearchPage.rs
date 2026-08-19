use crate::model::Publication::PublicationInfo::PublicationInfo;

#[derive(Debug)]
pub struct SearchPage {
    pub items: Vec<PublicationInfo>,
    pub total: Option<u64>,
    pub has_more: bool
}