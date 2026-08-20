use std::sync::Arc;
use std::time::Duration;
use crate::error::SupportError::ErrorSupport;
use crate::model::Query::Query::Query;
use crate::model::Query::SearchPage::SearchPage;
use crate::model::Query::Source::Source;
use crate::model::Query::SourceId::SourceId;

#[derive(Debug, Default)]
pub struct DblpDataSource{
    pub client: Arc<reqwest::blocking::Client>,
    pub base_url: String, // -> https://dblp.org/search/publ/api
}

pub trait Aux{
    fn get_client_http(&self) -> Arc<reqwest::blocking::Client>;
    fn get_base_url(&self) -> String;
}

impl Aux for DblpDataSource{
    fn get_client_http(&self) -> Arc<reqwest::blocking::Client> {
        self.client.clone()
    }
    fn get_base_url(&self) -> String {
        self.base_url.clone()
    }
}

impl Source for DblpDataSource {
    fn id(&self) -> SourceId {
        SourceId::Dblp
    }

    fn display_name(&self) -> &str {
        todo!()
    }

    fn search(&self, q: &Query, page: usize) -> Result<SearchPage, ErrorSupport> {
        //realizar a chamada http com reqwest e montar com builder por conta dos requisitos
        //de encoding da URL
        todo!()
    }

    fn min_interval(&self) -> Duration {
        todo!()
    }

    fn max_page_size(&self) -> usize {
        todo!()
    }

}