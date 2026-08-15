use crate::model::Dblp::DblpHitNestedContent::DblpHitNestedContent;

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum HitWrapper {
    TIPO_SIMPLES(DblpHitNestedContent),
    TIPO_COMPOSTO(Vec<DblpHitNestedContent>),
}