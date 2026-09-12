fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    // TODO: return the longer string slice
    let _ = (a, b);
    unimplemented!("Implement longest")
}

fn main() {
    println!("{}", longest("ab", "xyz"));
    println!("{}", longest("rustacean", "rust"));
}
