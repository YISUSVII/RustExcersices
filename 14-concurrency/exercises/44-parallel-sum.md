# Exercise 44 — Parallel Sum

- **Title:** Parallel Sum
- **Objective:** Split work across threads and join results.
- **Difficulty:** Advanced
- **Concepts practiced:** `thread::spawn`, `join`, moves

## Instructions

Implement `fn parallel_sum(data: Vec<i64>, workers: usize) -> i64` that partitions `data` into about `workers` chunks, sums each chunk on its own thread, then sums partial results.

## Expected behavior

- `parallel_sum((1..=1000).collect(), 4)` => `500500`

## Optional hints

- Handle `workers == 0` by treating as 1.
- Use `std::thread::scope` (Rust 1.63+) or owned chunks with `spawn`.

## Bonus challenge

Compare with a single-threaded baseline timing.
