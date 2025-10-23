use anyhow::Result;
use std::fs;

pub fn read_txt_file(file: &str) -> Result<String> {
    Ok(fs::read_to_string(file)?)
}

pub fn format_ukrainian_text(text: &str, alphabet: &str) -> String {
    text.to_lowercase()
        .replace("ґ", "г")
        .chars()
        .filter(|ch| alphabet.contains(*ch))
        .collect()
}

