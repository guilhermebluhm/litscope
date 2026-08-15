mod utils;
mod error;
mod model;
mod wrapper;

use crate::model::Publication::PublicationInfo::{PublicationInfo, Rules};
use crate::utils::HttpClient;

fn main() {

    let res = HttpClient::processar_requisicao("static+analysis+technical+debt",1);
    if let Ok(content) = res{
        println!("{:?}", content);
        let _ = PublicationInfo::normalize_and_produce_publication(content);
    }

}
