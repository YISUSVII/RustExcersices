# Exercise 24 — Text Analyzer

- **Title:** Text Analyzer
- **Objective:** Compute basic text statistics.
- **Difficulty:** Intermediate
- **Concepts practiced:** `String`, iterators, tuples

## Instructions

Implement `fn analyze(text: &str) -> (usize, usize, usize)` returning `(chars, words, lines)`.
- `chars` counts Unicode scalar values (use `text.chars().count()`)
- `words` are whitespace-separated tokens
- `lines` splits on `\n` (empty text => 0 lines)

## Expected behavior

- `"hello world\nrust"` => `(16, 3, 2)` characters may vary with newlines included

## Optional hints

- Empty string should return `(0, 0, 0)`.

## Bonus challenge

Also return the longest word.
