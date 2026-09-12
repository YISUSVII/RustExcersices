fn analyze(text: &str) -> (usize, usize, usize) {
    if text.is_empty() {
        return (0, 0, 0);
    }
    let chars = text.chars().count();
    let words = text.split_whitespace().count();
    let lines = text.lines().count();
    (chars, words, lines)
}

fn main() {
    let sample = "hello world\nrust";
    println!("{:?}", analyze(sample));
    println!("{:?}", analyze(""));
}
