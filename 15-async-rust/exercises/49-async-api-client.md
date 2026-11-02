# Exercise 49 — Async API Client

- **Title:** Async API Client
- **Objective:** Build an async API client over a fake in-memory HTTP transport, with timeouts (a select-like future) and retries with backoff.
- **Difficulty:** Ninja
- **Concepts practiced:** async traits via boxed futures, `Pin<Box<dyn Future + Send + 'a>>`, select/timeout futures, retry policies, error enums, `async fn` methods

> **Real projects use a runtime and an HTTP crate.** With Tokio + reqwest this is `tokio::time::timeout(dur, reqwest::get(url)).await`. Here everything is std-only: the `block_on`/`sleep` runtime from exercise 47 is provided, and the "network" is an in-memory fake, so you can focus on how timeouts and retries are built from futures.

## Instructions

1. Implement the timeout combinator:
   ```rust
   struct Timeout<F> { inner: Pin<Box<F>>, deadline: Sleep }
   fn timeout<F: Future>(duration: Duration, fut: F) -> Timeout<F>
   impl<F: Future> Future for Timeout<F> { type Output = Result<F::Output, Elapsed>; ... }
   ```
   Poll `inner` first (`Ready` → `Ok`), then `deadline` (`Ready` → `Err(Elapsed)`).
2. Use the "async trait" pattern:
   ```rust
   type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
   trait Transport { fn send(&self, req: Request) -> BoxFuture<'_, Result<Response, TransportError>>; }
   ```
3. Implement `Transport for FakeTransport`: increment `calls`, await `sleep(latency)`, fail with `TransportError::ConnectionReset` while `failures_left > 0` (decrementing it), otherwise return the configured route or a `404`.
4. Implement `ApiClient<T: Transport> { transport, max_retries, timeout, backoff }` with:
   ```rust
   async fn get(&self, path: &str) -> Result<String, ApiError>
   ```
   - map results: `Elapsed` → `ApiError::Timeout`, transport error → `ApiError::Transport(e)`, `2xx` → `Ok(body)`, otherwise `ApiError::Status(code)`;
   - retry while `error.is_retryable()` (timeouts, transport errors, `5xx`) and `attempt < max_retries`, sleeping `backoff * attempt` between attempts.

## Expected behavior

- With 2 initial connection failures and `max_retries = 2`, `get("/users/1")` succeeds on the 3rd call.
- `get("/missing")` → `Err(Status(404))` after exactly **1** call (4xx is not retried).
- With server latency 100 ms, timeout 20 ms and `max_retries = 1` → `Err(Timeout)` after 2 calls.
- `timeout(50ms, async { 7 })` → `Ok(7)`.

## Optional hints

- `Box::pin(async move { ... })` turns an `async` block into the `BoxFuture` the trait returns; the `'_` lifetime lets the future borrow `self`.
- Native `async fn` in traits is stable since Rust 1.75, but such traits are not object-safe (`dyn Transport` won't work) — that's why crates like `async-trait` box the future, as done here.
- Struct-update syntax `ApiClient { timeout: ..., ..ApiClient::new(t) }` is handy for tests.

## Bonus challenge

The Tokio + reqwest equivalent:

```rust
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    for attempt in 0..3 {
        match tokio::time::timeout(Duration::from_secs(2), client.get("https://httpbin.org/get").send()).await {
            Ok(Ok(resp)) if resp.status().is_success() => { println!("{}", resp.text().await?); break; }
            _ => tokio::time::sleep(Duration::from_millis(100 * (attempt + 1))).await,
        }
    }
    Ok(())
}
```

Add a `get_many(&self, paths: &[&str])` that fetches all paths concurrently using your `join_all` from exercise 48, and add jitter to the backoff.
