use rand::Rng;

pub fn get_subtext(text: &String, subtext_length: usize) -> String {
    let mut rng = rand::thread_rng();
    
    let text_length = text.chars().count();
    let start = rng.gen_range(0..=text_length - subtext_length);
    let subtext = text.chars().skip(start).take(subtext_length).collect();

    return subtext;
}