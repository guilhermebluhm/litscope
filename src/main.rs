mod utils;
mod error;
mod model;
mod wrapper;

use std::sync::Arc;
use crate::error::AppError::AppError;
use crate::model::Publication::PublicationInfo::{PublicationInfo, Rules};
use crate::model::Query::DataWrappers::DblpSource::DblpDataSource;
use crate::model::Query::Query::Query;
use crate::model::Query::Source::SourceRegistry;

fn main() -> Result<(), AppError> {

    let mut pos:usize = 0;
    let args = std::env::args();
    let client_http = Arc::new(reqwest::blocking::Client::new());

    /*
      todo: adaptar logica para type source com N elementos (pois agora sempre eh: dblp)
      e talvez refatoração para baixar o acoplamento
    */

    if args.len() < 5 {
        return Err(AppError::Config("não foram informado os parametros para a pesquisa".to_string()))
    }

    let mut query:Query = Query::default();
    for i in args.into_iter().skip(1){
        match pos {
            0 => {
                query.phrase = Some(i);
            }
            1 => {
                let year = i.trim().parse::<u16>().unwrap_or_default();
                query.year_range = Some((year, year));
            }
            2 => {
                query.name = i;
            }
            3 => {
                query.type_source.push(i);
            }
            _ => ()
        };
        pos+=1;
    }

    let dblp:DblpDataSource = DblpDataSource{
        client: Arc::clone(&client_http),
        base_url: "https://dblp.org/search/publ/api".to_string()

    };

    let sources = SourceRegistry{
        registry: vec![Box::new(dblp)],
    };

    //implementar a chamada da instancia polimorfica do dblp(DblpSource)
    if let Some(x) = sources.registry.get(0){
        let _ = x.as_ref().search(&query, 0);
    }

    Ok(())

}
