use std::collections::HashMap;

fn stock_in(inv: &mut HashMap<String, i32>, sku: &str, qty: i32) {
    let _ = (inv, sku, qty);
    unimplemented!("Implement stock_in")
}

fn stock_out(inv: &mut HashMap<String, i32>, sku: &str, qty: i32) -> Result<(), String> {
    let _ = (inv, sku, qty);
    unimplemented!("Implement stock_out")
}

fn quantity(inv: &HashMap<String, i32>, sku: &str) -> i32 {
    let _ = (inv, sku);
    unimplemented!("Implement quantity")
}

fn main() {
    let mut inv = HashMap::new();
    stock_in(&mut inv, "apple", 5);
    stock_out(&mut inv, "apple", 2).unwrap();
    println!("apple={}", quantity(&inv, "apple"));
    println!("{:?}", stock_out(&mut inv, "apple", 10));
}
