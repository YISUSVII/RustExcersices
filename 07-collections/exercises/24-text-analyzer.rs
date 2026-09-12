fn analyze(text: &str) -> (usize, usize, usize) {
    // TODO: (chars, words, lines)
    let _ = text;
    unimplemented!("Implement analyze")
}

fn main() {
    let sample = "hello world\nrust";
    println!("{:?}", analyze(sample));
    println!("{:?}", analyze(""));
}
