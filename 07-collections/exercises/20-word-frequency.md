# Exercise 20 — Word Frequency

- **Title:** Word Frequency
- **Objective:** Count words with a `HashMap`.
- **Difficulty:** Intermediate
- **Concepts practiced:** `HashMap`, string splitting, entry API

## Instructions

Implement `fn word_frequency(text: &str) -> HashMap<String, usize>` counting whitespace-separated words (case-sensitive).

## Expected behavior

- `"to be or not to be"` => `to:2, be:2, or:1, not:1`

## Optional hints

- `counts.entry(word.to_string()).or_insert(0)` then increment.

## Bonus challenge

Normalize to lowercase and strip punctuation.
