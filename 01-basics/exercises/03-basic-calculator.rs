fn calculate(a: i32, b: i32, op: char) -> Option<i32> {
    // TODO: route operators to add/sub/mul/div functions
    // Suggested behavior:
    // '+' => Some(a + b)
    // '-' => Some(a - b)
    // '*' => Some(a * b)
    // '/' => None when b == 0
    let _ = (a, b, op);
    unimplemented!("Implement calculate using reusable functions")
}

fn main() {
    println!("{:?}", calculate(8, 2, '/'));
}
