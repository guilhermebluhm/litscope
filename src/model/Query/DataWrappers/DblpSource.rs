use std::sync::Arc;
use std::time::Duration;
use reqwest::Url;
use crate::error::SupportError::ErrorSupport;
use crate::model::Publication::PublicationInfo::{PublicationInfo, Rules};
use crate::model::Query::Query::Query;
use crate::model::Query::SearchPage::SearchPage;
use crate::model::Query::Source::Source;
use crate::model::Query::SourceId::SourceId;
use crate::utils::HttpClient;

#[derive(Debug, Default)]
pub struct DblpDataSource{
    pub client: Arc<reqwest::blocking::Client>,
    pub base_url: String, // -> https://dblp.org/search/publ/api
}

impl Source for DblpDataSource {
    fn id(&self) -> SourceId {
        SourceId::Dblp
    }

    fn display_name(&self) -> &str {
        todo!()
    }

    fn search(&self, q: &Query, page: usize) -> Result<SearchPage, Vec<ErrorSupport>> {

        //type_source do objeto query quando ajustada a logica. ajustar aqui também para suportar multiplas fontes
        let mut url = Url::parse("https://dblp.org").map_err(|e| vec![ErrorSupport::FalhaAoMontarQueryParam("falha ao montar a url".to_string())])?;
        url.path_segments_mut().map_err(|_| vec![ErrorSupport::FalhaAoMontarQueryParam("falha ao montar url".to_string())])?
            .push("search")
            .push("publ")
            .push("api");
        
        url.query_pairs_mut().clear()
            .append_pair("q", q.phrase.clone().unwrap().as_str())
            .append_pair("format", "json")
            .append_pair("h", "100")
            .append_pair("f", page.to_string().as_str());
        
        let retorno_requisicao = HttpClient::processar_requisicao(&self.client, &url, 5, "dblp");
        if let Err(e) = retorno_requisicao {
            return Err(e);
        }
        
        let dblp_publication = PublicationInfo::normalize_and_produce_publication(retorno_requisicao?, q.phrase.clone().unwrap().as_str());
        if let Err(e) = dblp_publication{
            return Err(vec![ErrorSupport::FalhaAoProcessarPublicationInfo(e)])
        }
        
        let page = SearchPage{
            items: dblp_publication.unwrap(),
            has_more: false,
        };
        
        Ok(page)
    }

    fn min_interval(&self) -> Duration {
        todo!()
    }

    fn max_page_size(&self) -> usize {
        todo!()
    }

}