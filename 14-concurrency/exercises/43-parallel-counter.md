# Exercise 43 — Parallel Counter

- **Title:** Parallel Counter
- **Objective:** Increment a shared counter from many threads safely, first with `Arc<Mutex<_>>`, then lock-free with `AtomicUsize`, and sum a slice in parallel with scoped threads.
- **Difficulty:** Intermediate
- **Concepts practiced:** `std::thread::spawn`, `JoinHandle`, `Arc<T>`, `Mutex<T>`, `AtomicUsize`, memory `Ordering`, `std::thread::scope`

## Instructions

1. Implement `fn count_with_mutex(threads: usize, increments: usize) -> usize` using `Arc<Mutex<usize>>` and `thread::spawn`. Join every handle before reading the result.
2. Implement `fn count_with_atomic(threads: usize, increments: usize) -> usize` using `AtomicUsize::fetch_add` inside `thread::scope` (no `Arc` needed).
3. Implement `fn parallel_sum(data: &[u64], threads: usize) -> u64` that splits `data` into chunks and sums each chunk on its own scoped thread.

## Expected behavior

- `count_with_mutex(8, 10_000)` → `80000`
- `count_with_atomic(8, 10_000)` → `80000`
- `parallel_sum(&(1..=1_000_000).collect::<Vec<_>>(), 4)` → `500000500000`
- `parallel_sum(&[], 4)` → `0`

## Optional hints

- `thread::spawn` requires `'static` closures, so you must `Arc::clone` and `move` the clone in.
- `thread::scope(|s| { s.spawn(|| ...); })` lets threads borrow local data because all of them are joined before `scope` returns.
- `Ordering::Relaxed` is enough for a pure counter: you only need atomicity, not ordering with other memory.
- `slice.chunks(n)` panics if `n == 0` — compute `data.len().div_ceil(threads.max(1))`.

## Bonus challenge

Time the three approaches with `std::time::Instant` for 8 threads × 1,000,000 increments and explain why the mutex version is slowest.
