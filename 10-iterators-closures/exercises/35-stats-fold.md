# Exercise 35 — Stats with Fold

- **Title:** Stats with Fold
- **Objective:** Aggregate values using `fold`.
- **Difficulty:** Advanced
- **Concepts practiced:** `fold`, tuples

## Instructions

Implement `fn sum_and_count(nums: &[i32]) -> (i32, usize)` using `fold` (not `sum`/`count` helpers).

## Expected behavior

- `[1, 2, 3]` => `(6, 3)`
- `[]` => `(0, 0)`

## Optional hints

- Accumulators can be tuples.

## Bonus challenge

Also track min and max in the same fold.
