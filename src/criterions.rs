use crate::utils;
use crate::freq;

pub fn criteria20(ukr_frequencies: &[(String, f64)], text_for_check: &String, num_top_lgrams: usize, l: usize, text_length: usize) -> bool {
    let ukr_top_frequencies = freq::get_often_lgrams(ukr_frequencies, num_top_lgrams);
    let lsequence = utils::get_subtext(text_for_check, text_length);
    let lsequence_frequencies = freq::calculate_frequencies(&lsequence, l, None);
    
    let result = freq::existence_check(&ukr_top_frequencies, &lsequence_frequencies);
    return result;
}

pub fn criteria21(ukr_frequencies: &[(String, f64)], text_for_check: &String, num_top_lgrams: usize, l: usize, text_length: usize, limit: usize) -> bool {
    let ukr_top_frequencies = freq::get_often_lgrams(ukr_frequencies, num_top_lgrams);
    let lsequence = utils::get_subtext(text_for_check, text_length);
    let lsequence_frequencies = freq::calculate_frequencies(&lsequence, l, None);
    let common_lgrams = freq::intersection_of_sets(&lsequence_frequencies, &ukr_top_frequencies);

    let result = common_lgrams.len() >= limit;
    return result;
}

pub fn criteria22(ukr_frequencies: &[(String, f64)], text_for_check: &String, num_top_lgrams: usize, l: usize, text_length: usize) -> bool {
    let ukr_top_frequencies = freq::get_often_lgrams(ukr_frequencies, num_top_lgrams);
    let lsequence = utils::get_subtext(text_for_check, text_length);
    let lsequence_frequencies = freq::calculate_frequencies(&lsequence, l, None);
    let common_lgrams = freq::intersection_of_sets(&lsequence_frequencies, &ukr_top_frequencies);

    for (lgram, freq) in &common_lgrams {
        let ukr_freq = ukr_top_frequencies.iter().find(|(ngram, _)| ngram == lgram).map(|(_, frequence)| *frequence).unwrap_or(0.0);
        if *freq < ukr_freq {
            return false;
        }
    }

    return true;
}

pub fn criteria23(ukr_frequencies: &[(String, f64)], text_for_check: &String, num_top_lgrams: usize, l: usize, text_length: usize) -> bool {
    let ukr_top_frequencies = freq::get_often_lgrams(ukr_frequencies, num_top_lgrams);
    let lsequence = utils::get_subtext(text_for_check, text_length);
    let lsequence_frequencies = freq::calculate_frequencies(&lsequence, l, None);
    let common_lgrams = freq::intersection_of_sets(&lsequence_frequencies, &ukr_top_frequencies);

    let (kf, ff) = freq::calculate_common_frequencies(&ukr_top_frequencies, &common_lgrams);
    let result = ff > kf;
    return result;
}

pub fn criteria40(text_for_check: &String, l: usize, text_length: usize, ukr_index: f64, index_difference: f64) -> bool {
    let lsequence = utils::get_subtext(text_for_check, text_length);
    let lsequence_index = utils::compute_index_of_coincidence(&lsequence, l);

    let result = (ukr_index - lsequence_index).abs() < index_difference;
    return result;
}

pub fn criteria50(ukr_frequencies: &[(String, f64)], text_for_check: &String, num_rare_lgrams: usize, l: usize, text_length: usize, num_of_empty_boxes: usize) -> bool {
    let ukr_rare_frequencies = freq::get_rare_lgrams(ukr_frequencies, num_rare_lgrams);
    let lsequence = utils::get_subtext(text_for_check, text_length);
    let lsequence_frequencies = freq::calculate_frequencies(&lsequence, l, None);
    let common_lgrams = freq::intersection_of_sets(&lsequence_frequencies, &ukr_rare_frequencies);

    let f_empty = num_rare_lgrams - common_lgrams.len();
    let result = f_empty > num_of_empty_boxes;
    return result;
}
