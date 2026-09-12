# Level 14 — Concurrency

## What you will learn

Threads, `std::thread`, move closures, channels, mutexes, `Arc`, shared state, Send/Sync.

## Simple explanation

Concurrency lets multiple tasks make progress, but shared state must be synchronized safely.

## Small syntax examples

```rust
let handle = std::thread::spawn(|| println!("from thread"));
handle.join().unwrap();
```

## Common mistakes

Data races via poor design, lock contention, panics on poisoned mutexes.

## Exercises

| # | Exercise | Starter |
| - | -------- | ------- |
| 44 | Parallel sum | `exercises/44-parallel-sum.rs` |
| 45 | Channel pipeline | `exercises/45-channel-pipeline.rs` |
| 46 | Mutex counter | `exercises/46-mutex-counter.rs` |

## Hints

Prefer message passing before shared mutable state.

## Bonus challenges

Benchmark threaded vs single-threaded versions.

## References

- <https://doc.rust-lang.org/book/ch16-00-concurrency.html>
