fn main() {
    let mut text = String::from("Rust");
    let len_ref = &text;

    // Intentional broken line for learning (E0502):
    text.push_str(" language");

    println!("Length before update: {}", len_ref.len());
    println!("Updated text: {text}");
}
