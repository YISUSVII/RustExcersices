fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    println!("{}", longest("ab", "xyz"));
    println!("{}", longest("rustacean", "rust"));
}
