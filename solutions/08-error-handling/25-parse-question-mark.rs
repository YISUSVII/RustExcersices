fn sum_csv(line: &str) -> Result<i32, String> {
    let mut total = 0;
    for part in line.split(',') {
        let n: i32 = part
            .trim()
            .parse()
            .map_err(|e| format!("parse error on '{part}': {e}"))?;
        total += n;
    }
    Ok(total)
}

fn main() {
    println!("{:?}", sum_csv("1, 2, 3"));
    println!("{:?}", sum_csv("1, x"));
}
