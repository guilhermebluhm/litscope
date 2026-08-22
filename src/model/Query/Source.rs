use std::time::Duration;
use crate::error::SupportError::ErrorSupport;
use crate::model::Query::Query::Query;
use crate::model::Query::SearchPage::SearchPage;
use crate::model::Query::SourceId::SourceId;

pub struct SourceRegistry{
    pub registry: Vec<Box<dyn Source>>,
}

pub trait Source {
    fn id(&self) -> SourceId;
    fn display_name(&self) -> &str;
    fn search(&self, q: &Query, page: usize) -> Result<SearchPage, Vec<ErrorSupport>>;
    fn min_interval(&self) -> Duration;
    fn max_page_size(&self) -> usize;
}