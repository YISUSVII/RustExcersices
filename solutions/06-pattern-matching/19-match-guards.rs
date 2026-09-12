fn classify(n: i32) -> &'static str {
    match n {
        0 => "zero",
        n if n < 0 && n % 2 == 0 => "negative even",
        n if n < 0 => "negative odd",
        n if n % 2 == 0 => "positive even",
        _ => "positive odd",
    }
}

fn main() {
    for n in [-4, -3, 0, 2, 7] {
        println!("{n}: {}", classify(n));
    }
}
