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

| # | Exercise | Starter |
| - | -------- | ------- |
| 47 | Async timer lab | `exercises/47-async-timer/` |
| 48 | Join concurrent work | `exercises/48-join-two.rs` |
| 49 | Async API client sketch | `exercises/49-async-client/` |

> Note: Tokio-based exercises need network-free local async practice first; HTTP client is optional/offline-mocked.

## Hints

Use Tokio runtime and avoid blocking calls in async contexts.

## Bonus challenges

Add retries and timeouts to async clients.

## References

- <https://rust-lang.github.io/async-book/>
