use modular_calculator::ops::{add, div};

fn main() {
    println!("2+3={}", add(2.0, 3.0));
    println!("9/3={:?}", div(9.0, 3.0));
    println!("9/0={:?}", div(9.0, 0.0));
}
