use anyhow::Result;
use prettytable::{Cell, Row, Table};
use rand::Rng;
use statistical_methods_in_cryptanalysis::{
    config::*, get_subtext, io, structural, tests, transform,
};

fn run_structural(formated_ukr_text: String) {
    //
    println!("L = {SUBTEXT_LEN}, l = {L}");

    println!(
        "orig: {}, ",
        1.0 - structural(
            &formated_ukr_text,
            SIZE_LIMIT,
            SUBTEXT_LEN,
            COMPRESSION_STRENGTH,
            RUNS
        )
    );

    // vigenere k = 1
    let vigenere_key = transform::generate_random_uniform_text(1, UKRAINIAN_ALPHABET);
    let vigenere_text_for_check =
        transform::vigenere_encryption(&formated_ukr_text, &vigenere_key, UKRAINIAN_ALPHABET);
    println!(
        "v, k = 1 {}, ",
        structural(
            &vigenere_text_for_check,
            SIZE_LIMIT,
            SUBTEXT_LEN,
            COMPRESSION_STRENGTH,
            RUNS
        )
    );

    // vigenere k = 5
    let vigenere_key = transform::generate_random_uniform_text(5, UKRAINIAN_ALPHABET);
    let vigenere_text_for_check =
        transform::vigenere_encryption(&formated_ukr_text, &vigenere_key, UKRAINIAN_ALPHABET);

    println!(
        "v, k = 5 {}, ",
        structural(
            &vigenere_text_for_check,
            SIZE_LIMIT,
            SUBTEXT_LEN,
            COMPRESSION_STRENGTH,
            RUNS
        )
    );

    // vigenere k = 10
    let vigenere_key = transform::generate_random_uniform_text(10, UKRAINIAN_ALPHABET);
    let vigenere_text_for_check =
        transform::vigenere_encryption(&formated_ukr_text, &vigenere_key, UKRAINIAN_ALPHABET);

    println!(
        "v, k = 10 {}, ",
        structural(
            &vigenere_text_for_check,
            SIZE_LIMIT,
            SUBTEXT_LEN,
            COMPRESSION_STRENGTH,
            RUNS
        )
    );

    // affine
    let random_affine_encryption_key = (
        rand::thread_rng().gen_range(0..(32usize.pow(1 as u32) / 2)) * 2 + 1,
        rand::thread_rng().gen_range(0..32usize.pow(1 as u32)),
    );
    let affine_text_for_check = transform::bigram_affine_encryption(
        &formated_ukr_text,
        random_affine_encryption_key,
        UKRAINIAN_ALPHABET,
    );
    println!(
        "affine l = 1 {}, ",
        structural(
            &affine_text_for_check,
            SIZE_LIMIT,
            SUBTEXT_LEN,
            COMPRESSION_STRENGTH,
            RUNS
        )
    );

    // affine l = 1
    let random_affine_encryption_key = (
        rand::thread_rng().gen_range(0..(32usize.pow(2 as u32) / 2)) * 2 + 1,
        rand::thread_rng().gen_range(0..32usize.pow(2 as u32)),
    );
    let affine_text_for_check = transform::bigram_affine_encryption(
        &formated_ukr_text,
        random_affine_encryption_key,
        UKRAINIAN_ALPHABET,
    );
    println!(
        "affine l = 2 {}, ",
        structural(
            &affine_text_for_check,
            SIZE_LIMIT,
            SUBTEXT_LEN,
            COMPRESSION_STRENGTH,
            RUNS
        )
    );

    // bigram affine l =1
    let random_bigram_affine_encryption_key = (
        rand::thread_rng().gen_range(0..(32usize.pow(1 as u32) / 2)) * 2 + 1,
        rand::thread_rng().gen_range(0..32usize.pow(1 as u32)),
    );
    let bigram_affine_text_for_check = transform::bigram_affine_encryption(
        &formated_ukr_text,
        random_bigram_affine_encryption_key,
        UKRAINIAN_ALPHABET,
    );
    println!(
        "b affine l = 1 {}, ",
        structural(
            &bigram_affine_text_for_check,
            SIZE_LIMIT,
            SUBTEXT_LEN,
            COMPRESSION_STRENGTH,
            RUNS
        )
    );

    // bigram affine l =1
    let random_bigram_affine_encryption_key = (
        rand::thread_rng().gen_range(0..(32usize.pow(2 as u32) / 2)) * 2 + 1,
        rand::thread_rng().gen_range(0..32usize.pow(2 as u32)),
    );
    let bigram_affine_text_for_check = transform::bigram_affine_encryption(
        &formated_ukr_text,
        random_bigram_affine_encryption_key,
        UKRAINIAN_ALPHABET,
    );
    println!(
        "b affine l = 2 {}, ",
        structural(
            &bigram_affine_text_for_check,
            SIZE_LIMIT,
            SUBTEXT_LEN,
            COMPRESSION_STRENGTH,
            RUNS
        )
    );

    // uniform random text
    let uniform_text_for_check =
        transform::generate_random_uniform_text(100 * SUBTEXT_LEN, UKRAINIAN_ALPHABET);
    println!(
        "rand {}, ",
        structural(
            &uniform_text_for_check,
            SIZE_LIMIT,
            SUBTEXT_LEN,
            COMPRESSION_STRENGTH,
            RUNS
        )
    );

    // recursive text
    let recursive_text_for_check =
        transform::generate_rec_text(100 * SUBTEXT_LEN, 1, UKRAINIAN_ALPHABET);
    println!(
        "rec l = 1 {}, ",
        structural(
            &recursive_text_for_check,
            SIZE_LIMIT,
            SUBTEXT_LEN,
            COMPRESSION_STRENGTH,
            RUNS
        )
    );

    // recursive text
    let recursive_text_for_check =
        transform::generate_rec_text(100 * SUBTEXT_LEN, 2, UKRAINIAN_ALPHABET);
    println!(
        "rec l = 2 {}, ",
        structural(
            &recursive_text_for_check,
            SIZE_LIMIT,
            SUBTEXT_LEN,
            COMPRESSION_STRENGTH,
            RUNS
        )
    );
}

fn main() -> Result<()> {
    let text = io::read_txt_file("gra-prestoliv.txt")?;
    let formated_ukr_text = io::format_ukrainian_text(&text, UKRAINIAN_ALPHABET);

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("test"),
        Cell::new("2.0"),
        Cell::new("2.1"),
        Cell::new("2.2"),
        Cell::new("2.3"),
        Cell::new("4.0"),
        Cell::new("5.0"),
        Cell::new("structural"),
    ]));

    // natural text
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
    table.add_row(make_row("natural", &results_fpp));

    // vigenere k = 1
    let vigenere_key = transform::generate_random_uniform_text(1, UKRAINIAN_ALPHABET);
    let vigenere_text_for_check =
        transform::vigenere_encryption(&formated_ukr_text, &vigenere_key, UKRAINIAN_ALPHABET);
    let results_fnp_vigenere_1 = tests::fnp(
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
    table.add_row(make_row("vigenere k = 1", &results_fnp_vigenere_1));

    // vigenere k = 5
    let vigenere_key = transform::generate_random_uniform_text(5, UKRAINIAN_ALPHABET);
    let vigenere_text_for_check =
        transform::vigenere_encryption(&formated_ukr_text, &vigenere_key, UKRAINIAN_ALPHABET);
    let results_fnp_vigenere_5 = tests::fnp(
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
    table.add_row(make_row("vigenere k = 5", &results_fnp_vigenere_5));

    // vigenere k = 10
    let vigenere_key = transform::generate_random_uniform_text(10, UKRAINIAN_ALPHABET);
    let vigenere_text_for_check =
        transform::vigenere_encryption(&formated_ukr_text, &vigenere_key, UKRAINIAN_ALPHABET);
    let results_fnp_vigenere_10 = tests::fnp(
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
    table.add_row(make_row("vigenere k = 10", &results_fnp_vigenere_10));

    // affine
    let random_affine_encryption_key = (
        rand::thread_rng().gen_range(0..(32usize / 2)) * 2 + 1,
        rand::thread_rng().gen_range(0..32usize),
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
    table.add_row(make_row("affine", &results_fnp_affine));

    // bigram affine
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
    table.add_row(make_row("bigram_affine", &results_fnp_bigram_affine));

    // uniform random text
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
    table.add_row(make_row("uniform_text", &results_fnp_uniform_text));

    // recursive text
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
    table.add_row(make_row("recursive_text", &results_fnp_recursive_text));

    // recursive text
    let nonsensical_text = "ау".repeat(SUBTEXT_LEN * 100);
    let bigram_affine = transform::bigram_affine_encryption(
        &nonsensical_text,
        random_bigram_affine_encryption_key,
        UKRAINIAN_ALPHABET,
    );

    println!("{}", bigram_affine.get(0..1000).unwrap());

    let results_fnp_recursive_text = tests::fnp(
        RUNS,
        SUBTEXT_LEN,
        &text,
        &bigram_affine,
        L,
        NUM_TOP_LGRAMS,
        LIMIT_TOP_LGRAMS,
        INDEX_DIFFERENCE,
        NUM_RARE_LGRAMS,
        NUM_OF_EMPTY_BOXES,
    );
    table.add_row(make_row("recursive_text", &results_fnp_recursive_text));

    table.printstd();

    structural(&bigram_affine, 0.55, 10000, 9, 1000);

    Ok(())
}

fn make_row(name: &str, results: &[f64; 7]) -> Row {
    let mut cells = vec![Cell::new(name)];
    cells.extend(results.iter().map(|r| {
        let msg = if (*r == 0.0) || (*r == 1.0) {
            format!("{:.0}", r)
        } else {
            format!("{:.4}", r)
        };
        let cell = Cell::new(msg.as_str());
        cell.with_style(prettytable::Attr::ForegroundColor(2))
    }));
    Row::new(cells)
}
