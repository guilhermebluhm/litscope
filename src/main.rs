mod utils;
mod error;
mod model;
mod wrapper;

use crate::error::AppError::AppError;
use crate::model::Publication::PublicationInfo::{PublicationInfo, Rules};
use crate::utils::HttpClient;

fn main() -> Result<(), AppError> {

    let mut it:usize = 0;
    let mut args_normalized:String = String::new();
    let mut args = std::env::args();

    if args.len() < 1 {
        return Err(AppError::Config("não foram informado os parametros para a pesquisa".to_string()))
    }
    else{
        if let Some(x) = args.nth(1){

            let args_value:Vec<&str> = x.split(" ").collect::<Vec<&str>>();
            for i in args_value.iter() {
                it+=1;
                args_normalized.push_str(i);
                if args_value.len() != it {
                    args_normalized.push_str("+");
                }
            }

        }
    }
    let res = HttpClient::processar_requisicao(args_normalized.as_str(),1);
    if let Ok(content) = res{
        if let Ok(x) = PublicationInfo::normalize_and_produce_publication(content, args_normalized.as_str()){
            println!("{:?}", x);
        }
    }

    Ok(())

}
