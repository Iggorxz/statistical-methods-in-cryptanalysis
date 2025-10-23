use crate::freq;
use rand::Rng;

pub fn get_subtext(text: &str, subtext_length: usize) -> &str {
    let mut rng = rand::thread_rng();

    let text_length = text.len() / 2;
    let start = rng.gen_range(0..=text_length - subtext_length) * 2;
    &text[start..(start + subtext_length * 2)]
}

pub fn compute_index_of_coincidence(text: &str, l: usize) -> f64 {
    let frequencies = freq::calculate_frequencies(text, l, None);
    let text_length = text.chars().count() as f64;
    let mut sum_freq = 0.0;
    let m = 1.0 + text_length - l as f64;

    for (_, freq) in frequencies {
        sum_freq += freq * (freq * m - 1.0) * m
    }

    sum_freq / (text_length * (text_length - 1.0))
}

pub fn int_to_symbols(mut int: usize, l: usize, ukrainian_alphabet: &str) -> String {
    let mut result = Vec::with_capacity(l);
    for _ in 0..l {
        let index = int % 32;
        let char = ukrainian_alphabet.chars().nth(index).expect("Error");
        result.push(char);
        int /= 32;
    }
    result.iter().rev().collect()
}

pub fn symbols_to_int(text: &str, ukrainian_alphabet: &str) -> usize {
    let mut num = 0;
    for ch in text.chars() {
        if let Some(index) = ukrainian_alphabet.chars().position(|c| c == ch) {
            num = num * 32 + index;
        }
    }
    num
}
