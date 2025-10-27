use anyhow::Result;
use rand::Rng;
use statistical_methods_in_cryptanalysis::{config::*, io, tests, transform};

fn main() -> Result<()> {
    let text = io::read_txt_file("gra-prestoliv.txt")?;
    let formated_ukr_text = io::format_ukrainian_text(&text, UKRAINIAN_ALPHABET);

    let results_fpp = tests::fpp(
        RUNS,
        SUBTEXT_LEN,
        &text,
        L,
        NUM_TOP_LGRAMS,
        LIMIT_TOP_LGRAMS,
        INDEX_DIFFERENCE,
        NUM_RARE_LGRAMS,
        NUM_OF_EMPTY_BOXES,
        UKRAINIAN_ALPHABET,
    );
    println!("{:?}", results_fpp);

    let vigenere_key =
        transform::generate_random_uniform_text(VIGENERE_KEY_LEN, UKRAINIAN_ALPHABET);
    let vigenere_text_for_check =
        transform::vigenere_encryption(&formated_ukr_text, &vigenere_key, UKRAINIAN_ALPHABET);
    let results_fnp_vigenere = tests::fnp(
        RUNS,
        SUBTEXT_LEN,
        &text,
        &vigenere_text_for_check,
        L,
        NUM_TOP_LGRAMS,
        LIMIT_TOP_LGRAMS,
        INDEX_DIFFERENCE,
        NUM_RARE_LGRAMS,
        NUM_OF_EMPTY_BOXES,
    );
    println!("{:?}", results_fnp_vigenere);

    let random_affine_encryption_key = (
        rand::thread_rng().gen_range(0..(32usize.pow(L as u32) / 2)) * 2 + 1,
        rand::thread_rng().gen_range(0..32usize.pow(L as u32)),
    );
    let affine_text_for_check = transform::bigram_affine_encryption(
        &formated_ukr_text,
        random_affine_encryption_key,
        UKRAINIAN_ALPHABET,
    );
    let results_fnp_affine = tests::fnp(
        RUNS,
        SUBTEXT_LEN,
        &text,
        &affine_text_for_check,
        L,
        NUM_TOP_LGRAMS,
        LIMIT_TOP_LGRAMS,
        INDEX_DIFFERENCE,
        NUM_RARE_LGRAMS,
        NUM_OF_EMPTY_BOXES,
    );
    println!("{:?}", results_fnp_affine);

    let random_bigram_affine_encryption_key = (
        rand::thread_rng().gen_range(0..(32usize.pow(L as u32) / 2)) * 2 + 1,
        rand::thread_rng().gen_range(0..32usize.pow(L as u32)),
    );
    let bigram_affine_text_for_check = transform::bigram_affine_encryption(
        &formated_ukr_text,
        random_bigram_affine_encryption_key,
        UKRAINIAN_ALPHABET,
    );
    let results_fnp_bigram_affine = tests::fnp(
        RUNS,
        SUBTEXT_LEN,
        &text,
        &bigram_affine_text_for_check,
        L,
        NUM_TOP_LGRAMS,
        LIMIT_TOP_LGRAMS,
        INDEX_DIFFERENCE,
        NUM_RARE_LGRAMS,
        NUM_OF_EMPTY_BOXES,
    );
    println!("{:?}", results_fnp_bigram_affine);

    let uniform_text_for_check =
        transform::generate_random_uniform_text(100 * SUBTEXT_LEN, UKRAINIAN_ALPHABET);
    let results_fnp_uniform_text = tests::fnp(
        RUNS,
        SUBTEXT_LEN,
        &text,
        &uniform_text_for_check,
        L,
        NUM_TOP_LGRAMS,
        LIMIT_TOP_LGRAMS,
        INDEX_DIFFERENCE,
        NUM_RARE_LGRAMS,
        NUM_OF_EMPTY_BOXES,
    );
    println!("{:?}", results_fnp_uniform_text);

    let recursive_text_for_check =
        transform::generate_rec_text(100 * SUBTEXT_LEN, L, UKRAINIAN_ALPHABET);

    let results_fnp_recursive_text = tests::fnp(
        RUNS,
        SUBTEXT_LEN,
        &text,
        &recursive_text_for_check,
        L,
        NUM_TOP_LGRAMS,
        LIMIT_TOP_LGRAMS,
        INDEX_DIFFERENCE,
        NUM_RARE_LGRAMS,
        NUM_OF_EMPTY_BOXES,
    );
    println!("{:?}", results_fnp_recursive_text);

    Ok(())
}
