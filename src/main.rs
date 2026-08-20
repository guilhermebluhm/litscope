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

    if args.len() < 5 {
        return Err(AppError::Config("não foram informado os parametros para a pesquisa".to_string()))
    }
    else{
        let mut query:Query = Query::default();
        for i in args.into_iter().skip(1){
            match pos {
                0 => {
                    let args_value:Vec<&str> = i.split(" ").collect::<Vec<&str>>();
                    for i in args_value.iter() {
                        query.terms.push(String::from(*i));
                    }
                }
                1 => {
                    query.phrase = Some(i);
                }
                2 => {
                    let year = i.trim().parse::<u16>().unwrap_or_default();
                    query.year_range = Some((year, year));
                }
                3 => {
                    query.name = i;
                }
                4 => {
                    //por enquanto e apenas um source sendo recebido
                    //todo: adicionar logica para montar o type source para N elementos
                    query.type_source.push(i);
                }
                _ => ()
            };
            pos+=1;
        }

        /*
        todo: adaptar logica para type source com N elementos (pois agora sempre eh: dblp)
              e talvez refatoração para baixar o acoplamento
        */
        if query.type_source.get(0).unwrap().eq_ignore_ascii_case("dblp"){
            let dblp:DblpDataSource = DblpDataSource{
                client: Arc::clone(&client_http),
                base_url: "https://dblp.org/search/publ/api".to_string()

            };
            let sources = SourceRegistry{
                registry: vec![Box::new(dblp)],
            };
            
            //implementar a chamada da instancia polimorfica do dblp(DblpSource)
            
        }

/*        if let Some(x) = args.nth(1){

            let args_value:Vec<&str> = x.split(" ").collect::<Vec<&str>>();
            for i in args_value.iter() {
                it+=1;
                args_normalized.push_str(i);
                if args_value.len() != it {
                    args_normalized.push_str("+");
                }
            }

        }*/
    }
/*    let res = HttpClient::processar_requisicao(args_normalized.as_str(),1);
    if let Ok(content) = res{
        if let Ok(x) = PublicationInfo::normalize_and_produce_publication(content, args_normalized.as_str()){
            println!("{:?}", x);
        }
    }*/

    Ok(())

}
