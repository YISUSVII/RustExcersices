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
- [47 — Async Timer](exercises/47-async-timer.md)
- [48 — Concurrent Requests](exercises/48-concurrent-requests.md)
- [49 — Async API Client](exercises/49-async-api-client.md)

See `exercises/` for starters and `../solutions/15-async-rust/` for reference solutions.

## Hints
Use Tokio runtime and avoid blocking calls in async contexts.

## Bonus challenges
Add retries and timeouts to async clients.

## References
<https://rust-lang.github.io/async-book/>
