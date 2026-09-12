# Exercise 27 — Custom Error Enum

- **Title:** Custom Error Enum
- **Objective:** Model multiple failure modes with an enum.
- **Difficulty:** Intermediate
- **Concepts practiced:** custom errors, `From`, `Display` optional

## Instructions

1. Define `enum AppError { Empty, NotANumber, TooBig }`.
2. Implement `fn parse_score(raw: &str) -> Result<u8, AppError>`:
   - empty => `Empty`
   - non-integer => `NotANumber`
   - value > 100 => `TooBig`
   - else `Ok(value as u8)` (value must fit `u8` and be <= 100)

## Expected behavior

- `"85"` => `Ok(85)`
- `""` => `Err(Empty)`
- `"x"` => `Err(NotANumber)`
- `"250"` => `Err(TooBig)` (or NotANumber if parse fails for overflow — treat >100 as TooBig after successful parse into a larger int)

## Optional hints

- Parse to `u32` first, then range-check.

## Bonus challenge

Implement `std::fmt::Display` for `AppError`.
