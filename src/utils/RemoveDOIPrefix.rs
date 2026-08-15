pub fn remove_prefix(content: &String) -> String {
    
    let mut content:String = String::new();
    
    if let Some(prefix) = content.strip_prefix("doi:") {
        content = prefix.to_string();
    }
    if let Some(x) = content.strip_prefix("https://doi.org/") {
        content = x.to_string();
    }
    
    content
    
}