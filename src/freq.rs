use itertools::Itertools;
use std::collections::{HashMap, HashSet};

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

pub fn existence_check(set1: &[(String, f64)], set2: &[(String, f64)]) -> bool {
    let lgrams2: HashSet<&String> = set2.iter().map(|(lgram, _)| lgram).collect();
    let verify = set1.iter().all(|(lgram, _)| lgrams2.contains(lgram));
    return verify;
}

pub fn intersection_of_sets(set1: &[(String, f64)], set2: &[(String, f64)]) -> Vec<(String, f64)> {
    let mut result = Vec::new();
    let lgrams1: HashSet<&String> = set1.iter().map(|(lgram, _)| lgram).collect();
    let lgrams2: HashSet<&String> = set2.iter().map(|(lgram, _)| lgram).collect();
    let common_lgrams: HashSet<&String> = lgrams1.intersection(&lgrams2).cloned().collect();

    for (lgram, freq) in set1.iter() {
        if common_lgrams.contains(lgram) {
            result.push((lgram.clone(), *freq));
        }
    }
    
    return result;
}

pub fn calculate_common_frequencies(set1: &[(String, f64)], set2: &[(String, f64)]) -> (f64, f64) {
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    
    let lgrams1: HashSet<&String> = set1.iter().map(|(lgram, _)| lgram).collect();
    let lgrams2: HashSet<&String> = set2.iter().map(|(lgram, _)| lgram).collect();
    let common_lgrams: HashSet<&String> = lgrams1.intersection(&lgrams2).cloned().collect();

    for (lgram, freq) in set1.iter() {
        if common_lgrams.contains(lgram) {
            sum1 += freq
        }
    } 

    for (lgram, freq) in set2.iter() {
        if common_lgrams.contains(lgram) {
            sum2 += freq
        }
    } 

    return (sum1, sum2);
}
