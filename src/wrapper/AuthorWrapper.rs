use crate::model::Dblp::Author::Author;
use crate::model::Dblp::DblpInfoAuthor::DblpInfoAuthor;

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AuthorWrapper {
    TIPO_SIMPLES(Author),
    TIPO_COMPOSTO(Vec<Author>),
}