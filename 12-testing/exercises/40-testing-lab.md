# Exercise 40 — Testing Lab Crate

- **Title:** Testing Lab
- **Objective:** Combine unit and integration tests in a Cargo project.
- **Difficulty:** Advanced
- **Concepts practiced:** unit tests, integration tests, `cargo test`

## Instructions

1. Implement `parse_pair` in `src/lib.rs`.
2. Keep unit tests in the same file.
3. Make integration tests in `tests/integration.rs` pass.
4. Run `cargo test` inside `exercises/40-testing-lab`.

## Expected behavior

- `"2x3"` => `Ok((2, 3))`
- `"nope"` => `Err`

## Optional hints

- Split on `'x'` once.

## Bonus challenge

Support whitespace around numbers.
