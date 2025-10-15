use crate::utils;
use crate::freq;

pub fn criteria20(ukr_frequencies: &[(String, f64)], text_for_check: &String, num_top_lgrams: usize, l: usize, text_length: usize) -> bool {
    let ukr_top_frequencies = freq::get_often_lgrams(ukr_frequencies, num_top_lgrams);
    let lsequence = utils::get_subtext(text_for_check, text_length);
    let lsequence_frequencies = freq::calculate_frequencies(&lsequence, l, None);
    
    let result = freq::existence_check(&ukr_top_frequencies, &lsequence_frequencies);
    return result;
}
