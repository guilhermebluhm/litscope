use std::time::Duration;
use reqwest::blocking::{Client};
use crate::error::SupportError::ErrorSupport;
use crate::error::SupportError::ErrorSupport::FalhaGeralProcessarRequisicaoHttp;
use crate::model::Dblp::DblpResult::DblpResult;

pub fn processar_requisicao(criterio: &str, iteracoes: u8) -> Result<DblpResult, Vec<ErrorSupport>> {

    let mut errors:Vec<ErrorSupport> = Vec::new();
    let mut json_content:DblpResult = DblpResult::default();

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build().unwrap();

    for i in 0..iteracoes {
        match client.get(format!("https://dblp.org/search/publ/api?q={}&format=json&h=10&f=0", criterio)).send() {
            Ok(response) => {
                if response.status().is_server_error(){
                    errors.push(FalhaGeralProcessarRequisicaoHttp(response.status().as_u16()));
                }
                if response.status().is_client_error(){
                    errors.push(FalhaGeralProcessarRequisicaoHttp(response.status().as_u16()));
                    break;
                }
                if response.status().is_success(){

                    let text_content = response.text().unwrap();
                    let json = serde_json::from_str::<DblpResult>(text_content.as_str());
                    if let Ok(json_data) = json {
                        json_content = json_data;
                    }

                }
            }
            Err(err) => { //TODO MELHORAR A LOGICA DO CLIENT AQUI NO BRAÇO DO ERR
                if err.is_timeout(){
                    errors.push(FalhaGeralProcessarRequisicaoHttp(err.status().unwrap().as_u16()))
                }
                if err.is_connect(){
                    errors.push(FalhaGeralProcessarRequisicaoHttp(err.status().unwrap().as_u16()))
                }
            }
        }
        std::thread::sleep(Duration::from_secs(5))
    }

    if !errors.is_empty() {
        return Err(errors)
    }

    Ok(json_content)

}