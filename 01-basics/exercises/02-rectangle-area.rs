fn area(width: f64, height: f64) -> f64 {
    // TODO bonus: return Result instead of 0.0 for invalid dimensions
    if width < 0.0 || height < 0.0 {
        0.0
    } else {
        width * height
    }
}

fn main() {
    let width = 5.0;
    let height = 3.0;
    println!("Area: {}", area(width, height));
}
