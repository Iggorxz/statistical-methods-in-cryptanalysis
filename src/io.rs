use std::fs;
use anyhow::Result;

pub fn read_txt_file(file: &str) -> Result<String> {
    let text = fs::read_to_string(file)?;
    return Ok(text); 
}
