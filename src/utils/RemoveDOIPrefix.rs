pub fn remove_prefix(value_doi: &String) -> String {
    
    let mut content:String = String::new();
    
    if let Some(prefix) = value_doi.strip_prefix("doi:") {
        content = prefix.to_string();
    }
    if let Some(x) = value_doi.strip_prefix("https://doi.org/") {
        content = x.to_string();
    }

    if(content.is_empty()) {
        content = value_doi.to_string();
    }

    content
    
}