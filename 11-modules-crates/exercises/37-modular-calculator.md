# Exercise 37 — Modular Calculator Library

- **Title:** Multi-module Calculator
- **Objective:** Split logic across modules in a small Cargo library.
- **Difficulty:** Advanced
- **Concepts practiced:** `mod`, `pub`, Cargo lib + bin

## Instructions

1. Open `exercises/37-modular-calculator/`.
2. Implement `ops::add/sub/mul/div` in `src/ops.rs`.
3. Re-export them from `src/lib.rs`.
4. Call them from `src/main.rs`.
5. Run with `cargo run` inside the project folder.

## Expected behavior

- `div(9.0, 3.0)` => `Some(3.0)`
- `div(9.0, 0.0)` => `None`

## Optional hints

- `pub mod ops;` and `pub use ops::*;`.

## Bonus challenge

Add a `parse` module that turns `"1 + 2"` into an operation.
