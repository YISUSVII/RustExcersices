fn area(width: f64, height: f64) -> f64 {
    // TODO: reject negative numbers by returning 0.0 or handling with Result in bonus
    width * height
}

fn main() {
    let width = 5.0;
    let height = 3.0;
    println!("Area: {}", area(width, height));
}
