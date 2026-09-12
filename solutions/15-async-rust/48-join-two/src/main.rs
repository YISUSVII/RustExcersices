async fn sq(n: i32) -> i32 {
    n * n
}

async fn sum_two(a: i32, b: i32) -> i32 {
    let (x, y) = tokio::join!(sq(a), sq(b));
    x + y
}

#[tokio::main]
async fn main() {
    println!("{}", sum_two(3, 4).await);
}
