# Exercise 11 — Safe String Slices

- **Title:** Safe String Slices
- **Objective:** Return string slices without dangling references.
- **Difficulty:** Intermediate
- **Concepts practiced:** slices, lifetimes, borrowing, `find`

## Instructions

1. Implement `first_word(s: &str) -> &str` that returns the text before the first space, or the whole string if there is no space.
2. Implement `nth_word(s: &str, n: usize) -> Option<&str>` that returns the n-th whitespace-separated word (0-based), or `None` if missing.
3. Call both helpers from `main` with sample input.

## Expected behavior

- `first_word("hello rust world")` => `"hello"`
- `first_word("solo")` => `"solo"`
- `nth_word("hello rust world", 1)` => `Some("rust")`
- `nth_word("hello rust world", 5)` => `None`

## Optional hints

- Prefer `&str` parameters over `String` so callers keep ownership.
- `str::find` and slicing (`&s[..i]`) are enough for `first_word`.
- Split on whitespace for `nth_word`.

## Bonus challenge

Return a `Vec<&str>` of all words without allocating new `String`s.
