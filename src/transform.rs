use num_integer::Integer;
use crate::utils;

pub fn vigenere_encryption(text: &String, key: &String, alphabet: &str) -> String {
    let mut result = String::new();
    let alphabet_chars: Vec<char> = alphabet.chars().collect();
    let key_indices: Vec<usize> = key.chars().filter_map(|k| alphabet_chars.iter().position(|&c| c == k)).collect();
    let key_len = key_indices.len();

    for (i, ch) in text.chars().enumerate() {
        if let Some(text_index) = alphabet_chars.iter().position(|&c| c == ch) {
            let key_index = key_indices[i % key_len];
            let new_index = (text_index + key_index) % alphabet_chars.len();
            result.push(alphabet_chars[new_index]); 
        }
    }

    return result;
}

pub fn affine_encryption(text: &String, key: (usize, usize), alphabet: &str) -> String {
    let mut result = String::new();

    if key.0.gcd(&32) != 1 {
        panic!("a and m are not comprime");
    }

    for char in text.chars() {
        let x = utils::symbols_to_int(char.to_string(), alphabet);
        let y = (key.0 * x + key.1) % 32;
        let new_char = utils::int_to_symbols(y, 1, alphabet);
        result.push_str(&new_char);
    }

    return result;
}