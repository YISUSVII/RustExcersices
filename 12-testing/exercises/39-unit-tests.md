# Exercise 39 — Unit Tests for Math

- **Title:** Unit Tests
- **Objective:** Implement functions and prove them with `#[test]`.
- **Difficulty:** Advanced
- **Concepts practiced:** `#[cfg(test)]`, `assert_eq!`, `assert!`

## Instructions

1. Implement `add`, `saturating_sub_u8`, and `is_palindrome`.
2. Fill in the unit tests module so `rustc --test` / included tests pass conceptually.
3. Run with `rustc --test 39-unit-tests.rs && ./39-unit-tests` or convert to a cargo project if preferred.

## Expected behavior

- Tests cover happy paths and edge cases (empty palindrome, saturation at 0).

## Optional hints

- `#[should_panic]` is optional here; prefer `Result`-free asserts.

## Bonus challenge

Add table-driven tests with arrays of cases.
