#![allow(clippy::too_many_arguments)]
use crate::{
    config::{COMPRESSION_STRENGTH, SIZE_LIMIT},
    criterions, freq, io, structural_criteria, utils,
};

pub fn fpp(
    runs: usize,
    subtext_len: usize,
    ukr_text: &str,
    l: usize,
    num_top_lgrams: usize,
    limit_top_lgrams: usize,
    index_difference: f64,
    num_rare_lgrams: usize,
    num_of_empty_boxes: usize,
    alphabet: &'static str,
) -> (f64, f64, f64, f64, f64, f64, f64) {
    let formated_text = io::format_ukrainian_text(ukr_text, alphabet);
    let ukr_frequencies = freq::calculate_frequencies(&formated_text, l, None);
    let ukr_index = utils::compute_index_of_coincidence(&formated_text, l);

    let mut true_criteria20: f64 = 0.0;
    let mut true_criteria21: f64 = 0.0;
    let mut true_criteria22: f64 = 0.0;
    let mut true_criteria23: f64 = 0.0;
    let mut true_criteria40: f64 = 0.0;
    let mut true_criteria50: f64 = 0.0;
    let mut true_structural_criteria: f64 = 0.0;

    for _ in 0..runs {
        if criterions::criteria20(
            &ukr_frequencies,
            &formated_text,
            num_top_lgrams,
            l,
            subtext_len,
        ) {
            true_criteria20 += 1.0;
        }

        if criterions::criteria21(
            &ukr_frequencies,
            &formated_text,
            num_top_lgrams,
            l,
            subtext_len,
            limit_top_lgrams,
        ) {
            true_criteria21 += 1.0;
        }

        if criterions::criteria22(
            &ukr_frequencies,
            &formated_text,
            num_top_lgrams,
            l,
            subtext_len,
        ) {
            true_criteria22 += 1.0;
        }

        if criterions::criteria23(
            &ukr_frequencies,
            &formated_text,
            num_top_lgrams,
            l,
            subtext_len,
        ) {
            true_criteria23 += 1.0;
        }

        if criterions::criteria40(&formated_text, l, subtext_len, ukr_index, index_difference) {
            true_criteria40 += 1.0;
        }

        if criterions::criteria50(
            &ukr_frequencies,
            &formated_text,
            num_rare_lgrams,
            l,
            subtext_len,
            num_of_empty_boxes,
        ) {
            true_criteria50 += 1.0;
        }

        if structural_criteria(
            &formated_text,
            SIZE_LIMIT,
            subtext_len,
            COMPRESSION_STRENGTH,
        ) {
            true_structural_criteria += 1.0
        };
    }

    (
        1.0 - (true_criteria20 / runs as f64),
        1.0 - (true_criteria21 / runs as f64),
        1.0 - (true_criteria22 / runs as f64),
        1.0 - (true_criteria23 / runs as f64),
        1.0 - (true_criteria40 / runs as f64),
        1.0 - (true_criteria50 / runs as f64),
        1.0 - (true_structural_criteria / runs as f64),
    )
}

pub fn fnp(
    runs: usize,
    subtext_len: usize,
    formated_text: &str,
    text_for_check: &str,
    l: usize,
    num_top_lgrams: usize,
    limit_top_lgrams: usize,
    index_difference: f64,
    num_rare_lgrams: usize,
    num_of_empty_boxes: usize,
) -> (f64, f64, f64, f64, f64, f64, f64) {
    let ukr_frequencies = freq::calculate_frequencies(formated_text, l, None);
    let ukr_index = utils::compute_index_of_coincidence(formated_text, l);

    let mut true_criteria20: f64 = 0.0;
    let mut true_criteria21: f64 = 0.0;
    let mut true_criteria22: f64 = 0.0;
    let mut true_criteria23: f64 = 0.0;
    let mut true_criteria40: f64 = 0.0;
    let mut true_criteria50: f64 = 0.0;
    let mut true_structural_criteria: f64 = 0.0;

    for _ in 0..runs {
        if criterions::criteria20(
            &ukr_frequencies,
            text_for_check,
            num_top_lgrams,
            l,
            subtext_len,
        ) {
            true_criteria20 += 1.0;
        }

        if criterions::criteria21(
            &ukr_frequencies,
            text_for_check,
            num_top_lgrams,
            l,
            subtext_len,
            limit_top_lgrams,
        ) {
            true_criteria21 += 1.0;
        }

        if criterions::criteria22(
            &ukr_frequencies,
            text_for_check,
            num_top_lgrams,
            l,
            subtext_len,
        ) {
            true_criteria22 += 1.0;
        }

        if criterions::criteria23(
            &ukr_frequencies,
            text_for_check,
            num_top_lgrams,
            l,
            subtext_len,
        ) {
            true_criteria23 += 1.0;
        }

        if criterions::criteria40(text_for_check, l, subtext_len, ukr_index, index_difference) {
            true_criteria40 += 1.0;
        }

        if criterions::criteria50(
            &ukr_frequencies,
            text_for_check,
            num_rare_lgrams,
            l,
            subtext_len,
            num_of_empty_boxes,
        ) {
            true_criteria50 += 1.0;
        }
        if structural_criteria(
            text_for_check,
            SIZE_LIMIT,
            subtext_len,
            COMPRESSION_STRENGTH,
        ) {
            true_structural_criteria += 1.0
        };
    }

    (
        true_criteria20 / runs as f64,
        true_criteria21 / runs as f64,
        true_criteria22 / runs as f64,
        true_criteria23 / runs as f64,
        true_criteria40 / runs as f64,
        true_criteria50 / runs as f64,
        true_structural_criteria / runs as f64,
    )
}
