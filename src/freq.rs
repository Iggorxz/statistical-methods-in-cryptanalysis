use itertools::Itertools;
use std::collections::HashMap;

pub fn calculate_frequencies(text: &String, l: usize, alphabet: Option<&str>) -> Vec<(String, f64)> {
    let mut frequencies = HashMap::new();
    let chars: Vec<char> = text.chars().collect();
    let total_lgrams = (chars.len() - l + 1) as f64;

    if let Some(alphabet_str) = alphabet {
        let alphabet_chars: Vec<char> = alphabet_str.chars().collect();
        for combo in std::iter::repeat(alphabet_chars).take(l).multi_cartesian_product() {
            let lgram: String = combo.into_iter().collect();
            frequencies.insert(lgram, 0);
        }
    }

    for window in chars.windows(l) {
        let lgram: String = window.iter().collect();
        *frequencies.entry(lgram).or_insert(0) += 1;
    }

    let mut sorted_lgrams: Vec<(String, u32)> = frequencies.into_iter().collect();
    sorted_lgrams.sort_by(|a, b| b.1.cmp(&a.1));
    
    let result = sorted_lgrams.into_iter().map(|(lgram, count)| (lgram, count as f64 / total_lgrams)).collect();
    return result;

}

pub fn get_often_lgrams(frequencies: &[(String, f64)], num: usize) -> Vec<(String, f64)> {
    let often_lgrams = frequencies.iter().take(num).cloned().collect();
    return often_lgrams;
}

pub fn get_rare_lgrams(frequencies: &[(String, f64)], num: usize) -> Vec<(String, f64)> {
    let rare_lgrams = frequencies[frequencies.len() - num..].to_vec();
    return rare_lgrams;
}