fn area(width: f64, height: f64) -> f64 {
    if width < 0.0 || height < 0.0 {
        0.0
    } else {
        width * height
    }
}

fn main() {
    println!("Area: {}", area(5.0, 3.0));
}
