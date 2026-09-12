# Exercise 25 — Parse with `?`

- **Title:** Parse Pipeline
- **Objective:** Chain fallible operations with `?`.
- **Difficulty:** Intermediate
- **Concepts practiced:** `Result`, `?`, `parse`

## Instructions

Implement `fn sum_csv(line: &str) -> Result<i32, String>` that splits on commas, parses each trimed token as `i32`, and returns the sum. Convert parse errors into `String`.

## Expected behavior

- `"1, 2, 3"` => `Ok(6)`
- `"1, x"` => `Err(...)`

## Optional hints

- Map parse errors with `.map_err(|e| e.to_string())?`.

## Bonus challenge

Ignore empty tokens between commas.
