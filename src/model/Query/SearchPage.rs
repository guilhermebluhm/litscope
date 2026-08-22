use crate::model::Publication::PublicationInfo::PublicationInfo;

#[derive(Debug, Default)]
pub struct SearchPage {
    pub items: Vec<PublicationInfo>,
    pub has_more: bool, //avaliar a necessidade do atributo total (removido temporariamente)
}