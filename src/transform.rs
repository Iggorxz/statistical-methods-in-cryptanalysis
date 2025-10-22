use num_integer::Integer;

pub fn vigenere_encryption(text: &String, key: &String, alphabet: &str) -> String {
    let mut result = String::new();
    let key_indices: Vec<usize> = key.chars().filter_map(|k| alphabet.chars().position(|c| c == k)).collect();
    let key_len = key.chars().count();

    for (i, ch) in text.chars().enumerate() {
        if let Some(text_index) = alphabet.chars().position(|c| c == ch) {
            let key_index = key_indices[i % key_len];
            let new_index = (text_index + key_index) % 32;
            let new_char = alphabet.chars().nth(new_index)                
            .expect("index is out of alphabet");
            result.push(new_char); 
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
        let x: usize = alphabet.chars().position(|c| c == char)
        .expect("Character not found in alphabet");
        let y = (key.0 * x + key.1) % 32;
        let new_char = alphabet.chars().nth(y)
        .expect("index is out of alphabet");
        result.push(new_char);
    }

    return result;
}