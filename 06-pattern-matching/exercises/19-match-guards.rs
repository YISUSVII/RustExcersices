fn classify(n: i32) -> &'static str {
    // TODO: use match + guards
    let _ = n;
    unimplemented!("Implement classify")
}

fn main() {
    for n in [-4, -3, 0, 2, 7] {
        println!("{n}: {}", classify(n));
    }
}
