use std::collections::HashMap;

fn add_score(book: &mut HashMap<String, Vec<u32>>, name: &str, score: u32) {
    let _ = (book, name, score);
    unimplemented!("Implement add_score")
}

fn average(book: &HashMap<String, Vec<u32>>, name: &str) -> Option<f64> {
    let _ = (book, name);
    unimplemented!("Implement average")
}

fn main() {
    let mut book = HashMap::new();
    add_score(&mut book, "Ada", 80);
    add_score(&mut book, "Ada", 100);
    println!("{:?}", average(&book, "Ada"));
    println!("{:?}", average(&book, "Missing"));
}
