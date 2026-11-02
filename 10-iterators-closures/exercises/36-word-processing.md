# Exercise 36 — Word Processing

- **Title:** Word Processing
- **Objective:** Process text with iterator chains and build your own iterator types and adaptors.
- **Difficulty:** Advanced
- **Concepts practiced:** `split_whitespace`, `chars`, `flat_map`, `windows`, `peekable`, implementing `Iterator`, extension traits, returning `impl Iterator<Item = &str> + '_`

## Instructions

1. `normalize_words(text: &str) -> impl Iterator<Item = String> + '_` — split on whitespace, strip leading/trailing non-alphanumeric characters (keep inner `'`), lowercase, and skip empty results.
2. `word_counts(text: &str) -> BTreeMap<String, usize>` — built from `normalize_words`.
3. `top_n(text: &str, n: usize) -> Vec<(String, usize)>` — most frequent words; ties broken alphabetically.
4. `capitalize_words(text: &str) -> String` — `"hello big world"` → `"Hello Big World"`.
5. `long_words(text: &str, min_len: usize) -> impl Iterator<Item = &str> + '_` — words (with surrounding punctuation trimmed) of at least `min_len` chars; borrows from `text` (no allocation).
6. `bigrams(text: &str) -> Vec<(String, String)>` — consecutive normalized word pairs (hint: `windows(2)`).
7. A custom iterator `Sentences<'a>` that yields trimmed sentences from a `&'a str`, splitting on `.`, `!` and `?` (the terminator stays with the sentence). Provide `fn sentences(text: &str) -> Sentences<'_>`.
8. A custom **adaptor**: `struct Chunked<I> { inner: I, size: usize }` that groups items into `Vec<I::Item>` chunks, plus an extension trait
   ```rust
   trait ChunkedExt: Iterator + Sized {
       fn chunked(self, size: usize) -> Chunked<Self>;
   }
   impl<I: Iterator> ChunkedExt for I { /* ... */ }
   ```
   so you can write `words.chunked(3)`. A `size` of `0` should be treated as `1`.

## Expected behavior

- `normalize_words("Hello, world! It's   fine.")` → `["hello", "world", "it's", "fine"]`
- `top_n("a b a c b a", 2)` → `[("a", 3), ("b", 2)]`
- `capitalize_words("hello big world")` → `"Hello Big World"`
- `long_words("a tiny elephant!", 4)` → `["tiny", "elephant"]`
- `bigrams("one two three")` → `[("one","two"), ("two","three")]`
- `sentences("Hi there. How are you? Great!")` → `["Hi there.", "How are you?", "Great!"]`
- `(1..=7).chunked(3)` → `[[1,2,3], [4,5,6], [7]]`

## Optional hints

- `str::trim_matches(|c: char| !c.is_alphanumeric() && c != '\'')`.
- For `capitalize_words`, take `chars()`, uppercase the first with `to_uppercase()`, and `chain` the rest.
- In `Sentences::next`, find the first terminator with `find(['.', '!', '?'])`, slice, and advance the remaining `&'a str`.
- In `Chunked::next`, use `self.inner.by_ref().take(self.size).collect()` and return `None` for an empty chunk.

## Bonus challenge

Implement a `Interleave<A, B>` adaptor that alternates items from two iterators until both are exhausted.
