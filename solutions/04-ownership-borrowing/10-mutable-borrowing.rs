fn append_suffix(text: &mut String, suffix: &str) {
    text.push_str(suffix);
}

fn main() {
    let mut text = String::from("Rust");

    let before_len = text.len();
    append_suffix(&mut text, " language");

    println!("Length before update: {before_len}");
    println!("Updated text: {text}");
}
