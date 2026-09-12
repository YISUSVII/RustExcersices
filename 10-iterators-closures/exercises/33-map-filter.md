# Exercise 33 — Map and Filter

- **Title:** Map and Filter
- **Objective:** Transform collections with iterator adapters.
- **Difficulty:** Advanced
- **Concepts practiced:** `map`, `filter`, `collect`

## Instructions

Implement `fn squares_of_evens(nums: &[i32]) -> Vec<i32>` keeping even numbers and squaring them, preserving order.

## Expected behavior

- `[1, 2, 3, 4]` => `[4, 16]`

## Optional hints

- `filter(|n| n % 2 == 0).map(|n| n * n)`.

## Bonus challenge

Do it in one pass without intermediate allocations beyond the output vec.
