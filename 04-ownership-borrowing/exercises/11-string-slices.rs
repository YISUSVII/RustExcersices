fn first_word(s: &str) -> &str {
    // TODO: return text before first space, or whole string
    let _ = s;
    unimplemented!("Implement first_word")
}

fn nth_word(s: &str, n: usize) -> Option<&str> {
    // TODO: return the n-th whitespace-separated word (0-based)
    let _ = (s, n);
    unimplemented!("Implement nth_word")
}

fn main() {
    let sentence = "hello rust world";
    println!("first: {}", first_word(sentence));
    println!("nth 1: {:?}", nth_word(sentence, 1));
    println!("nth 5: {:?}", nth_word(sentence, 5));
}
