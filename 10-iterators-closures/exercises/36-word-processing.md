# Exercise 36 — Word Processing

- **Title:** Word Processing
- **Objective:** Chain iterator adapters for text.
- **Difficulty:** Advanced
- **Concepts practiced:** `flat_map`, `filter`, `collect`

## Instructions

Implement `fn long_words(text: &str, min_len: usize) -> Vec<String>` returning lowercase words whose length >= `min_len`.

## Expected behavior

- `"Rust is Amazing"`, min 4 => `["rust", "amazing"]`

## Optional hints

- `split_whitespace`, `to_lowercase`, `filter`.

## Bonus challenge

Sort unique results alphabetically.
