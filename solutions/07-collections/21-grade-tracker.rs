use std::collections::HashMap;

fn add_score(book: &mut HashMap<String, Vec<u32>>, name: &str, score: u32) {
    book.entry(name.to_string()).or_default().push(score);
}

fn average(book: &HashMap<String, Vec<u32>>, name: &str) -> Option<f64> {
    let scores = book.get(name)?;
    if scores.is_empty() {
        return None;
    }
    let sum: u32 = scores.iter().sum();
    Some(sum as f64 / scores.len() as f64)
}

fn main() {
    let mut book = HashMap::new();
    add_score(&mut book, "Ada", 80);
    add_score(&mut book, "Ada", 100);
    println!("{:?}", average(&book, "Ada"));
    println!("{:?}", average(&book, "Missing"));
}
