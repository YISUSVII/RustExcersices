fn calculate(a: i32, b: i32, op: char) -> Option<i32> {
    match op {
        '+' => Some(a + b),
        '-' => Some(a - b),
        '*' => Some(a * b),
        '/' if b != 0 => Some(a / b),
        _ => None,
    }
}

fn main() {
    println!("{:?}", calculate(8, 2, '/'));
}
