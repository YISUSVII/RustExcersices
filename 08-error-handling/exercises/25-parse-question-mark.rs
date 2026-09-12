fn sum_csv(line: &str) -> Result<i32, String> {
    // TODO: split, parse each part, sum with ?
    let _ = line;
    unimplemented!("Implement sum_csv")
}

fn main() {
    println!("{:?}", sum_csv("1, 2, 3"));
    println!("{:?}", sum_csv("1, x"));
}
