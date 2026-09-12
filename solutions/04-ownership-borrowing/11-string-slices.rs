fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn nth_word(s: &str, n: usize) -> Option<&str> {
    s.split_whitespace().nth(n)
}

fn main() {
    let sentence = "hello rust world";
    println!("first: {}", first_word(sentence));
    println!("nth 1: {:?}", nth_word(sentence, 1));
    println!("nth 5: {:?}", nth_word(sentence, 5));
}
