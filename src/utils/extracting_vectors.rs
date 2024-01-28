
use regex::Regex;
pub fn extracting_type(contents: &str) -> Vec<String> {
    let mut new_vector: Vec<String> = Vec::new();
let regex = Regex::new(r#"type="([^"]*)""#).unwrap();

    for captures in regex.captures_iter(contents) {
        if let Some(type_match) = captures.get(1) {
            let type_value = type_match.as_str();
            new_vector.push(String::from(type_value));
        }else{
            println!("No match found");
        }
    }
    new_vector
}

pub fn extracting_name(contents: &str) -> Vec<String> {
    let mut new_vector: Vec<String> = Vec::new();
    let regex = Regex::new(r#"value="([^"]*)""#).unwrap();
    for captures in regex.captures_iter(contents) {
        if let Some(name_match) = captures.get(1) {
            let name_value = name_match.as_str();
            new_vector.push(String::from(name_value));
        }else{
            new_vector.push("NIL".to_string());
        }
    }
    new_vector
}

