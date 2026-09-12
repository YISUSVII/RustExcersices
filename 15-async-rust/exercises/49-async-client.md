# Exercise 49 — Async Client Sketch (Offline)

- **Title:** Async API Client Sketch
- **Objective:** Structure async client code without requiring network access.
- **Difficulty:** Advanced
- **Concepts practiced:** async traits-like APIs, `Result`, mock transport

## Instructions

1. Implement `async fn fetch_message(transport: &impl Transport) -> Result<String, String>`.
2. `Transport::get(&self, path: &str) -> Result<String, String>` is async via async-trait pattern simulated with an enum mock.
3. Complete the mock so `/hello` returns `"hi"`.

## Expected behavior

- Mock transport `/hello` => `Ok("hi")`.

## Optional hints

- Keep it std + tokio only; no real HTTP required.

## Bonus challenge

Add timeout using `tokio::time::timeout`.
