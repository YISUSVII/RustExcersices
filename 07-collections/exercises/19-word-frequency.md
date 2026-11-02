# Exercise 19 — Word Frequency

- **Title:** Word Frequency Counter
- **Objective:** Count how often each word appears in a text and report the most common ones.
- **Difficulty:** Intermediate
- **Concepts practiced:** `HashMap`, the `entry` API, string normalization, sorting with `sort_by`

## Instructions

1. Implement `word_frequencies(text: &str) -> HashMap<String, usize>`:
   - split on whitespace,
   - strip leading/trailing non-alphanumeric characters (`"rust!"` → `"rust"`),
   - lowercase every word,
   - skip words that become empty.
2. Implement `top_n(freqs: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)>` sorted by count (descending), then alphabetically for ties.

## Expected behavior

For the text `"The Rust book. Rust is fast; the borrow checker is the friend."` and `n = 3`:

```text
the: 3
is: 2
rust: 2
```

`is` and `rust` both appear twice, so the tie is broken alphabetically.

## Optional hints

- `*map.entry(word).or_insert(0) += 1;`
- `str::trim_matches(|c: char| !c.is_alphanumeric())`
- `b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))`

## Bonus challenge

Make `word_frequencies` return `HashMap<&str, usize>` borrowing from the input when no lowercasing is needed (hint: `Cow<str>`).
