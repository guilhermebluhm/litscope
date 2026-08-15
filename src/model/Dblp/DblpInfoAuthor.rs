use crate::model::Dblp::Author::Author;
use crate::wrapper::AuthorWrapper::AuthorWrapper;

#[derive(serde::Deserialize, Debug, Default, Clone)]
pub struct DblpInfoAuthor{
    pub author: Option<AuthorWrapper>,
}