fn add(a: f64, b: f64) -> f64 { a + b }
fn sub(a: f64, b: f64) -> f64 { a - b }
fn mul(a: f64, b: f64) -> f64 { a * b }
fn div(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn execute(a: f64, b: f64, op: char) -> Option<f64> {
    match op {
        '+' => Some(add(a, b)),
        '-' => Some(sub(a, b)),
        '*' => Some(mul(a, b)),
        '/' => div(a, b),
        _ => None,
    }
}

fn main() {
    println!("{:?}", execute(9.0, 3.0, '/'));
}
