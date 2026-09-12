use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let counts = word_frequency("to be or not to be");
    println!("{counts:?}");
}
