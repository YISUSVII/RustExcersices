# Exercise 10 — Mutable Borrowing (Broken Code)

- **Title:** Mutable Borrowing String Editor
- **Objective:** Fix borrowing issues while updating a `String`.
- **Difficulty:** Intermediate
- **Concepts practiced:** mutable references, borrowing rules, `E0502`

## Instructions

The starter code intentionally borrows the same string immutably and mutably at the same time.
Reorder or refactor code so it compiles and prints both length and updated text.

## Expected behavior

- Compiles without borrow checker errors
- Prints original length and updated string

## Optional hints

- End immutable borrow before mutable borrow starts.

## Bonus challenge

Create a function `append_suffix(text: &mut String, suffix: &str)` and reuse it.
