pub fn check_valid(content: &String) -> bool {

    /*

    importante:

    strip_prefix -> realiza o recorte da string de um determinado prefixo
    split_whitespaces -> aplica o trim de forma subjacente e separa por espaços
    split_once -> realiza o recorte da primeira ocorrencia dividindo em segmentos
    split -> vai produzir um iterador separando o conteudo em conjunto de &str que requer ser coletado
             e apos realizar a aplicação dos filtros desejados

    dif. split / split_once : enquanto o primeiro produz uma sequencia vai verbosa de todo o subconjunto
    daquele conteudo, a segunda vai buscar o pattern_matching em separar em uma tupla de dois segmentos
    o antes e depois do padrão identificado

    */

    let prefix = content.strip_prefix("doi:");
    if prefix.is_some(){
        //todo
    }
    else{

        if  content.contains("n/a") || content.contains("none") {
            return false
        }
        if let Some(x) = content.split_once("/"){
            if x.1.is_empty(){
                return false;
            }
            if !x.0.starts_with("10."){
                return false;
            }
        }
        else{
            return false;
        }
    }
    true
    
}