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
Parallel counter, producer/consumer, worker pool, shared-state simulation.

## Hints
Prefer message passing before shared mutable state.

## Bonus challenges
Benchmark threaded vs single-threaded versions.

## References
<https://doc.rust-lang.org/book/ch16-00-concurrency.html>
