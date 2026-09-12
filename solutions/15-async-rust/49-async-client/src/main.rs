struct MockTransport;

impl MockTransport {
    async fn get(&self, path: &str) -> Result<String, String> {
        match path {
            "/hello" => Ok(String::from("hi")),
            other => Err(format!("not found: {other}")),
        }
    }
}

async fn fetch_message(transport: &MockTransport) -> Result<String, String> {
    transport.get("/hello").await
}

#[tokio::main]
async fn main() {
    let t = MockTransport;
    println!("{:?}", fetch_message(&t).await);
}
