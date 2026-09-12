# Exercise 26 — Validate User Input

- **Title:** Input Validation
- **Objective:** Validate fields and aggregate errors.
- **Difficulty:** Intermediate
- **Concepts practiced:** `Result`, domain validation

## Instructions

Implement `fn validate_username(name: &str) -> Result<(), String>`:
- non-empty
- length <= 16
- only ASCII alphanumeric or `_`

## Expected behavior

- `"ada_lovelace"` => `Ok(())`
- `""` and `"bad name!"` => `Err`

## Optional hints

- Use `chars().all(...)`.

## Bonus challenge

Return all failed rules as a joined error string.
