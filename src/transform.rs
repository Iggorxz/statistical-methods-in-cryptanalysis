use num_integer::Integer;
use rand::Rng;
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

pub fn bigram_affine_encryption(text: &String, key: (usize, usize), alphabet: &str) -> String {
    let mut result = String::new();
    let mut text = text.clone();
    let m = alphabet.chars().count().pow(2);

    if text.chars().count() % 2 != 0 {
        text.push('а');
    }

    if key.0.gcd(&m) != 1 {
        panic!("a and m are not comprime");
    }

    for i in (0..text.chars().count()).step_by(2) {
        let ch1 = text.chars().nth(i).unwrap();
        let ch2 = text.chars().nth(i+1).unwrap();
        let x = utils::symbols_to_int(ch1.to_string() + &ch2.to_string(), alphabet);
        let y = (key.0 * x + key.1) % m;
        let new_bigram = utils::int_to_symbols(y, 2, alphabet);
        result.push_str(&new_bigram);
    }

    return result;
}

pub fn generate_random_uniform_text(l: usize, alphabet: &str) -> String {
    let mut rng = rand::thread_rng();
    let alphabet_chars: Vec<char> = alphabet.chars().collect();
    let mut result = String::with_capacity(l);

    for _ in 0..l {
        let index = rng.gen_range(0..alphabet_chars.len());
        result.push(alphabet_chars[index]);
    }

    return result;
}

pub fn generate_rec_text(length: usize, l: usize, alphabet: &str) -> String {
    let mut result = String::new();
    let mut y0 = generate_random_uniform_text(l, alphabet);
    let mut y1 = generate_random_uniform_text(l, alphabet);
    result.push_str(&y0);
    result.push_str(&y1);

    for _ in 0..(length - 2) {
        let x0 = utils::symbols_to_int(y0.clone(), alphabet);
        let x1 = utils::symbols_to_int(y1.clone(), alphabet);
        let y_int = (x0 + x1) % 32usize.pow(l as u32);
        let y = utils::int_to_symbols(y_int, l, alphabet);

        result.push_str(&y);
        y0 = y1;
        y1 = y;
    }

    return result;
}
