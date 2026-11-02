# Exercise 21 — Duplicate Remover

- **Title:** Duplicate Remover
- **Objective:** Remove and detect duplicates while preserving the original order.
- **Difficulty:** Beginner
- **Concepts practiced:** `HashSet`, `HashSet::insert` return value, `Vec::retain`, borrowed `&str` items

## Instructions

1. Implement `dedup_numbers(items: &[i32]) -> Vec<i32>` — keep the first occurrence of each value, preserving order.
2. Implement `dedup_words(text: &str) -> Vec<&str>` — split on whitespace and keep the first occurrence of each word, comparing **case-insensitively** but returning the original spelling.
3. Implement `find_duplicates(items: &[i32]) -> Vec<i32>` — values that appear more than once, each listed once, sorted ascending.
4. Implement `dedup_in_place(items: &mut Vec<i32>)` using `Vec::retain`.

## Expected behavior

```text
dedup_numbers: [3, 1, 4, 5, 9, 2, 6]
dedup_words: ["Rust", "is", "fun"]
find_duplicates: [1, 3, 5]
in place: [3, 1, 4, 5, 9, 2, 6]
```

## Optional hints

- `HashSet::insert` returns `false` when the value was already present.
- `Vec::dedup` only removes **consecutive** duplicates — not what we want here.

## Bonus challenge

Make `dedup_numbers` generic over `T: Eq + Hash + Clone`.
