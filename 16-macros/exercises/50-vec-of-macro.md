# Exercise 50 — vec_of! Macro

- **Title:** Declarative vec macro
- **Objective:** Write a `macro_rules!` that builds a vector.
- **Difficulty:** Advanced
- **Concepts practiced:** `macro_rules!`, repetition

## Instructions

Implement `macro_rules! vec_of` so `vec_of![1, 2, 3]` works like `vec![1, 2, 3]` for one or more comma-separated expressions, plus empty `vec_of![]`.

## Expected behavior

- `vec_of![1, 2, 3]` => `vec![1, 2, 3]`

## Optional hints

- Use `$( $x:expr ),* $(,)?`.

## Bonus challenge

Support `vec_of![x; n]` repeat syntax.
