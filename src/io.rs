use std::fs;
use anyhow::Result;

pub fn read_txt_file(file: &str) -> Result<String> {
    let text = fs::read_to_string(file)?;
    return Ok(text); 
}

pub fn format_ukrainian_text(text: String, alphabet: &str) -> String {
    let mut formated_text = text.to_lowercase().replace("ґ", "г");
    formated_text = formated_text.chars().filter(|ch| alphabet.contains(*ch)).collect();
    return formated_text;
}