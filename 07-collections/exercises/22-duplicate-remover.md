# Exercise 22 — Duplicate Remover

- **Title:** Duplicate Remover
- **Objective:** Deduplicate while preserving first-seen order.
- **Difficulty:** Intermediate
- **Concepts practiced:** `HashSet`, `Vec`

## Instructions

Implement `fn unique_in_order(items: Vec<i32>) -> Vec<i32>`.

## Expected behavior

- `[1, 2, 1, 3, 2]` => `[1, 2, 3]`

## Optional hints

- Track seen values in a `HashSet`.

## Bonus challenge

Make it generic over `T: Eq + Hash + Clone`.
