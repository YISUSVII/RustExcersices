# Level 15 — Async Rust

## What you will learn
`async`, `await`, futures, Tokio, async tasks, concurrent execution, basic HTTP requests.

## Simple explanation
Async handles many waiting tasks efficiently without one OS thread per task.

## Small syntax examples
```rust
async fn hello() { println!("hello async"); }
```

## Common mistakes
Blocking inside async code and confusing async concurrency with threads.

## Exercises
Async timer, concurrent requests, basic async API client.

## Hints
Use Tokio runtime and avoid blocking calls in async contexts.

## Bonus challenges
Add retries and timeouts to async clients.

## References
<https://rust-lang.github.io/async-book/>
