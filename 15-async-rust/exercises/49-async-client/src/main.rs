struct MockTransport;

impl MockTransport {
    async fn get(&self, path: &str) -> Result<String, String> {
        // TODO: return Ok("hi") for "/hello", else Err
        let _ = path;
        unimplemented!("Implement MockTransport::get")
    }
}

async fn fetch_message(transport: &MockTransport) -> Result<String, String> {
    // TODO: call transport.get("/hello")
    let _ = transport;
    unimplemented!("Implement fetch_message")
}

#[tokio::main]
async fn main() {
    let t = MockTransport;
    println!("{:?}", fetch_message(&t).await);
}
