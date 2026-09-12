use std::collections::HashMap;

fn stock_in(inv: &mut HashMap<String, i32>, sku: &str, qty: i32) {
    *inv.entry(sku.to_string()).or_insert(0) += qty;
}

fn stock_out(inv: &mut HashMap<String, i32>, sku: &str, qty: i32) -> Result<(), String> {
    let entry = inv.entry(sku.to_string()).or_insert(0);
    if *entry < qty {
        return Err(String::from("insufficient stock"));
    }
    *entry -= qty;
    Ok(())
}

fn quantity(inv: &HashMap<String, i32>, sku: &str) -> i32 {
    *inv.get(sku).unwrap_or(&0)
}

fn main() {
    let mut inv = HashMap::new();
    stock_in(&mut inv, "apple", 5);
    stock_out(&mut inv, "apple", 2).unwrap();
    println!("apple={}", quantity(&inv, "apple"));
    println!("{:?}", stock_out(&mut inv, "apple", 10));
}
