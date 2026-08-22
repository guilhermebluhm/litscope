use std::sync::Arc;
use std::time::Duration;
use reqwest::Url;
use crate::error::SupportError::ErrorSupport;
use crate::model::Dblp::DblpResult::DblpResult;

pub fn processar_requisicao(client: &Arc<reqwest::blocking::Client>, url: &Url, iteracoes: u8, type_source: &str) -> Result<DblpResult, Vec<ErrorSupport>> {

    //direcionar a requisicao baseado no type_source (por enquanto e dblp fixo)
    let mut error_occured:Vec<ErrorSupport> = vec![];
    let mut dblp_result:DblpResult = DblpResult::default();
    
    for i in 0..iteracoes {
        match client.get(url.as_ref()).send() { 
            Ok(resp) => {
                if resp.status().is_server_error(){
                    error_occured.push(ErrorSupport::FalhaGeralProcessarRequisicaoHttp("Server error".to_string()));
                    break;
                }
                if resp.status().is_client_error(){
                    error_occured.push(ErrorSupport::FalhaGeralProcessarRequisicaoHttp("Client error".to_string()));
                }
                if resp.status().is_success() {

                    if let Ok(x) = resp.text(){

                        let dblp = serde_json::from_str::<DblpResult>(x.as_str());
                        if let Ok(content) = dblp {
                            dblp_result = content;
                        }

                    }

                }
            }
            Err(e) => {
                if e.is_timeout() {
                    error_occured.push(ErrorSupport::FalhaGeralProcessarRequisicaoHttp("Timeout".to_string()));
                }
            }
        }
        std::thread::sleep(Duration::from_secs(5));
    }
    
    if !error_occured.is_empty() {
        return Err(error_occured);
    }
    
    Ok(dblp_result)
    
}