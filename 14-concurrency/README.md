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
- [43 — Parallel Counter](exercises/43-parallel-counter.md)
- [44 — Producer / Consumer](exercises/44-producer-consumer.md)
- [45 — Worker Pool](exercises/45-worker-pool.md)
- [46 — Shared State Simulation](exercises/46-shared-state-simulation.md)

See `exercises/` for starters and `../solutions/14-concurrency/` for reference solutions.

## Hints
Prefer message passing before shared mutable state.

## Bonus challenges
Benchmark threaded vs single-threaded versions.

## References
<https://doc.rust-lang.org/book/ch16-00-concurrency.html>
