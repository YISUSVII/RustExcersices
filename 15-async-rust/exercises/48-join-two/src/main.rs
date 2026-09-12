async fn sq(n: i32) -> i32 {
    n * n
}

async fn sum_two(a: i32, b: i32) -> i32 {
    // TODO: join sq(a) and sq(b) concurrently
    let _ = (a, b, sq);
    unimplemented!("Implement sum_two")
}

#[tokio::main]
async fn main() {
    println!("{}", sum_two(3, 4).await);
}
