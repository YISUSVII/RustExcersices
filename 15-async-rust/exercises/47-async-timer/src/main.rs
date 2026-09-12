use std::time::Duration;

async fn delayed_message(ms: u64, msg: &str) -> String {
    // TODO: sleep then return msg owned
    let _ = (ms, msg, Duration::from_millis);
    unimplemented!("Implement delayed_message")
}

#[tokio::main]
async fn main() {
    let out = delayed_message(10, "done").await;
    println!("{out}");
}
