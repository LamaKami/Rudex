extern crate regex;
use regex::Regex;

pub fn tokenize_filter_special_characters(text: &str) -> Vec<String>{
    //! Removes every character that is not in the english alphabet, a number or a space. Also maps every char to lower
    let re = Regex::new(r"[^a-zA-Z0-9 ]").unwrap();

    let preprocesses_text = re.replace_all(text, " ").to_string();

    let mut tokens: Vec<String> = preprocesses_text.split(" ").map(|s| s.to_string().to_lowercase()).collect();
    tokens = tokens.into_iter().filter(|i|i!="").collect();

    return tokens;
}


