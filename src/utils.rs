use rand::Rng;
use crate::freq;

pub fn get_subtext(text: &String, subtext_length: usize) -> String {
    let mut rng = rand::thread_rng();
    
    let text_length = text.chars().count();
    let start = rng.gen_range(0..=text_length - subtext_length);
    let subtext = text.chars().skip(start).take(subtext_length).collect();

    return subtext;
}

pub fn compute_index_of_coincidence(text: &str, l: usize) -> f64 {
    let frequencies = freq::calculate_frequencies(&text.to_string(), l, None);
    let text_length = text.chars().count() as f64;
    let mut sum_freq = 0.0;
    let m = 1.0 + text_length - l as f64;

    for (_, freq) in frequencies {
        sum_freq += freq * (freq * m - 1.0) * m
    }

    let index = sum_freq / (text_length * (text_length - 1.0));
    return index;
}
