use std::collections::HashSet;

fn unique_in_order(items: Vec<i32>) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for item in items {
        if seen.insert(item) {
            out.push(item);
        }
    }
    out
}

fn main() {
    println!("{:?}", unique_in_order(vec![1, 2, 1, 3, 2]));
}
