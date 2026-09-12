use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<String, usize> {
    // TODO: count whitespace-separated words
    let _ = text;
    unimplemented!("Implement word_frequency")
}

fn main() {
    let counts = word_frequency("to be or not to be");
    println!("{counts:?}");
}
