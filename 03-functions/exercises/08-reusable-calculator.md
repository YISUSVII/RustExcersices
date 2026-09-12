# Exercise 08 — Reusable Functions Calculator

- **Title:** Reusable Functions Calculator
- **Objective:** Split calculator logic into reusable functions.
- **Difficulty:** Beginner
- **Concepts practiced:** function decomposition, scope, return values

## Instructions

1. Create individual functions: add/sub/mul/div.
2. Create `execute(a, b, op)` calling the right function.
3. Keep `main` tiny.

## Expected behavior

- `execute(9, 3, '/')` returns `Some(3.0)`

## Optional hints

- Return `Option<f64>` to handle unsupported operators.

## Bonus challenge

Support chained operations from a vector of `(op, value)`.
