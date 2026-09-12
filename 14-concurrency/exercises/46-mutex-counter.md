# Exercise 46 — Mutex Counter

- **Title:** Shared Mutex Counter
- **Objective:** Safely share mutable state across threads.
- **Difficulty:** Advanced
- **Concepts practiced:** `Arc`, `Mutex`

## Instructions

Implement `fn bump_many(threads: usize, times: usize) -> usize` where each thread increments a shared counter `times` times.

## Expected behavior

- `bump_many(8, 1000)` => `8000`

## Optional hints

- `Arc<Mutex<usize>>`.

## Bonus challenge

Try `RwLock` and discuss tradeoffs in comments.
