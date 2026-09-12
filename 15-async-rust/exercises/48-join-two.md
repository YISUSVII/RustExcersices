# Exercise 48 — Join Concurrent Work

- **Title:** Join Two Async Jobs
- **Objective:** Run independent futures concurrently.
- **Difficulty:** Advanced
- **Concepts practiced:** `tokio::join!`

## Instructions

1. Open `exercises/48-join-two/`.
2. Implement `sum_two` using `tokio::join!` on `sq(a)` and `sq(b)`.
3. `cargo run` prints `25` for inputs 3 and 4.

## Expected behavior

- `sum_two(3, 4)` => `25`

## Optional hints

- `let (x, y) = tokio::join!(sq(a), sq(b)); x + y`

## Bonus challenge

Add a third value with `tokio::join!`.
