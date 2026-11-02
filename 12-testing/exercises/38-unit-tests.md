# Exercise 38 — Unit Tests

- **Title:** Unit Tests
- **Objective:** Write a thorough unit-test suite for already-implemented functions, covering happy paths, edge cases, error values and expected panics.
- **Difficulty:** Intermediate
- **Concepts practiced:** `#[cfg(test)]`, `#[test]`, `assert!`, `assert_eq!`, `assert_ne!`, custom failure messages, `#[should_panic(expected = "...")]`, tests returning `Result<(), E>`, `use super::*`

## Instructions

The starter file already contains working implementations of:

- `is_palindrome(s: &str) -> bool` — ignores case and non-alphanumeric characters.
- `divide(a: i32, b: i32) -> Result<i32, String>` — errors on division by zero and on overflow (`i32::MIN / -1`).
- `parse_rgb(hex: &str) -> Result<(u8, u8, u8), String>` — parses `"#rrggbb"` (the `#` is optional).
- `BoundedStack<T>` with `new(capacity: usize) -> Self` (**panics** if `capacity == 0`), `push(&mut self, item: T) -> Result<(), T>`, `pop(&mut self) -> Option<T>`, `peek(&self) -> Option<&T>`, `len(&self) -> usize`, `is_empty(&self) -> bool`.

Your job is to replace every `todo!()` in the `#[cfg(test)] mod tests` block:

1. `palindrome_ignores_case_and_punctuation` — at least one positive and one negative case, with a custom message on the negative assertion.
2. `palindrome_edge_cases` — empty string, punctuation-only string, a unicode word.
3. `divide_happy_path_truncates_toward_zero` — check `7 / 2` and `-7 / 2`.
4. `divide_reports_errors` — division by zero (exact message) and overflow (message *contains* `"overflow"`).
5. `parse_rgb_with_and_without_hash` — make the test return `Result<(), String>` and use `?` instead of `unwrap()`.
6. `parse_rgb_rejects_bad_input` — wrong length, non-hex digits, non-ASCII input.
7. `stack_is_lifo_and_bounded` — fill the stack, assert `push` hands the item back when full, then check `peek`, `len` and LIFO `pop` order down to `None`.
8. `stack_with_zero_capacity_panics` — add `#[should_panic(expected = "capacity must be greater than zero")]`.

Run your tests with:

```bash
rustc --edition 2021 --test 38-unit-tests.rs -o unit_tests && ./unit_tests
# or, inside a Cargo project:
cargo test
```

## Expected behavior

- Before you start: `test result: FAILED. 0 passed; 8 failed`.
- When you finish: `test result: ok. 8 passed; 0 failed`.
- Temporarily break an implementation (e.g. change `checked_div` to `/`) and confirm at least one test fails with a clear message.

## Optional hints

- `assert!(cond, "message with {}", value)` — the message is only built when the assertion fails.
- `assert_eq!(left, right)` prints both values on failure; your types need `Debug` + `PartialEq`.
- `unwrap_err()` is handy to inspect an error message: `assert!(err.contains("overflow"))`.
- A test returning `Result` fails if it returns `Err`; you cannot combine that with `#[should_panic]`.
- Filter tests by name: `./unit_tests divide` runs only tests whose name contains `divide`.

## Bonus challenge

- Add a deliberately slow test marked `#[ignore]` and run it with `./unit_tests --ignored`.
- Move the tests into a Cargo project's `tests/` folder as an **integration test** — what has to become `pub`?
