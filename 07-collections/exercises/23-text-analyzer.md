# Exercise 23 — Text Analyzer

- **Title:** Text Analyzer
- **Objective:** Compute statistics about a piece of text using `String`, `char` and collection APIs.
- **Difficulty:** Intermediate
- **Concepts practiced:** `chars()` vs bytes, `lines()`, `split_whitespace()`, `BTreeMap`, `Option`, building `String`s

## Instructions

1. Implement `analyze(text: &str) -> TextStats` filling:
   - `chars` — number of Unicode scalar values (`chars().count()`, **not** `len()`),
   - `words` — whitespace-separated words,
   - `lines` — number of lines,
   - `sentences` — count of `.`, `!` and `?` characters,
   - `longest_word: Option<String>` — first longest word with surrounding punctuation trimmed (`None` for empty text),
   - `avg_word_len: f64` — average length (in chars) of trimmed words, `0.0` for empty text.
2. Implement `letter_histogram(text: &str) -> BTreeMap<char, usize>` — lowercase alphabetic characters only.
3. Implement `capitalize_words(text: &str) -> String` — capitalize the first letter of every word, joined with single spaces.

## Expected behavior

```text
TextStats { chars: 52, words: 9, lines: 2, sentences: 2, longest_word: Some("Tienes"), avg_word_len: 4.56 }
top letters: [('e', 6), ('a', 5), ('s', 5)]
Rust Makes Café Code Safe
```

Output for `"Rust makes café code safe.\n¿Tienes tiempo para Rust?"` (`avg_word_len` rounded to 2 decimals in `main`).

(The sample text contains non-ASCII characters on purpose — check `chars` vs `len()`.)

## Optional hints

- `word.trim_matches(|c: char| !c.is_alphanumeric())`
- `max_by_key` returns the **last** max on ties — use a fold or `reduce` to keep the first.
- Capitalize with `first.to_uppercase().chain(rest)`.

## Bonus challenge

Return the top-N letters by frequency without collecting into an intermediate `Vec` twice.
