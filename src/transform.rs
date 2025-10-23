use crate::utils;
use num_integer::Integer;
use rand::Rng;

pub fn vigenere_encryption(text: &str, key: &str, alphabet: &str) -> String {
    let mut result = String::new();
    let alphabet_chars: Vec<char> = alphabet.chars().collect();
    let key_indices: Vec<usize> = key
        .chars()
        .filter_map(|k| alphabet_chars.iter().position(|&c| c == k))
        .collect();
    let key_len = key_indices.len();

    for (i, ch) in text.chars().enumerate() {
        if let Some(text_index) = alphabet_chars.iter().position(|&c| c == ch) {
            let key_index = key_indices[i % key_len];
            let new_index = (text_index + key_index) % alphabet_chars.len();
            result.push(alphabet_chars[new_index]);
        }
    }

    result
}

pub fn affine_encryption(text: &str, key: (usize, usize), alphabet: &str) -> String {
    let mut result = String::new();

    if key.0.gcd(&32) != 1 {
        panic!("a and m are not comprime");
    }

    for i in (0..(text.len() * 2)).step_by(2) {
        let x = utils::symbols_to_int(&text[i..(i + 1)], alphabet);
        let y = (key.0 * x + key.1) % 32;
        let new_char = utils::int_to_symbols(y, 1, alphabet);
        result.push_str(&new_char);
    }

    result
}

pub fn bigram_affine_encryption(text: &str, key: (usize, usize), alphabet: &str) -> String {
    let mut result = String::new();
    let mut text = text.to_string();
    let m = alphabet.chars().count().pow(2);

    if !text.chars().count().is_multiple_of(2) {
        text.push('а');
    }

    dbg!(&m);
    dbg!(key.0);
    if key.0.gcd(&m) != 1 {
        panic!("a and m are not comprime");
    }
    for i in (0..text.len()).step_by(4) {
        let symbol = &text[i..(i + 4)];
        let x = utils::symbols_to_int(symbol, alphabet);
        let y = (key.0 * x + key.1) % m;
        let new_bigram = utils::int_to_symbols(y, 2, alphabet);
        result.push_str(&new_bigram);
    }

    result
}

pub fn generate_random_uniform_text(l: usize, alphabet: &str) -> String {
    let mut rng = rand::thread_rng();
    let alphabet_chars: Vec<char> = alphabet.chars().collect();
    let mut result = String::with_capacity(l);

    for _ in 0..l {
        let index = rng.gen_range(0..alphabet_chars.len());
        result.push(alphabet_chars[index]);
    }

    result
}

pub fn generate_rec_text(length: usize, l: usize, alphabet: &str) -> String {
    let mut result = String::new();
    let mut y0 = generate_random_uniform_text(l, alphabet);
    let mut y1 = generate_random_uniform_text(l, alphabet);
    result.push_str(&y0);
    result.push_str(&y1);

    for _ in 0..(length - 2) {
        let x0 = utils::symbols_to_int(y0.as_str(), alphabet);
        let x1 = utils::symbols_to_int(y1.as_str(), alphabet);
        let y_int = (x0 + x1) % 32usize.pow(l as u32);
        let y = utils::int_to_symbols(y_int, l, alphabet);

        result.push_str(&y);
        y0 = y1;
        y1 = y.to_owned();
    }

    result
}
