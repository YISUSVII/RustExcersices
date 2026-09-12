use std::time::Duration;

async fn delayed_message(ms: u64, msg: &str) -> String {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    msg.to_string()
}

#[tokio::main]
async fn main() {
    let out = delayed_message(10, "done").await;
    println!("{out}");
}
