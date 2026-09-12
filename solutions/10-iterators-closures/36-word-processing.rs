fn long_words(text: &str, min_len: usize) -> Vec<String> {
    text.split_whitespace()
        .map(|w| w.to_lowercase())
        .filter(|w| w.len() >= min_len)
        .collect()
}

fn main() {
    println!("{:?}", long_words("Rust is Amazing", 4));
}
