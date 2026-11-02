# Exercise 32 — Reference-Returning APIs

- **Title:** Reference-Returning APIs
- **Objective:** Write functions and structs that return borrowed data, using explicit lifetime annotations to describe which input the output borrows from.
- **Difficulty:** Advanced
- **Concepts practiced:** lifetime parameters, lifetime elision rules, structs holding references, `&str` slicing, `Option<&T>`, iterators over borrowed data

## Instructions

1. Implement `longest<'a>(a: &'a str, b: &'a str) -> &'a str`.
   - On equal length, return `a`.
2. Implement `first_word(s: &str) -> &str` (lifetime elided — explain in a comment why that is allowed).
   - Leading whitespace is skipped; an empty/blank string returns `""`.
3. Implement `longest_with_prefix<'a>(words: &[&'a str], prefix: &str) -> Option<&'a str>`.
   - Note that the result borrows from `words`' contents, **not** from `prefix`.
4. Create a tokenizer struct that holds a reference:
   ```rust
   struct Parser<'a> { input: &'a str, pos: usize }
   ```
   with methods:
   - `fn new(input: &'a str) -> Self`
   - `fn next_token(&mut self) -> Option<&'a str>` — returns the next whitespace-separated token. The returned slice must outlive the `&mut self` borrow (that's why it is `&'a str`, not `&str`).
   - `fn remaining(&self) -> &'a str` — the unparsed rest (leading whitespace trimmed).
5. Implement `split_key_value(line: &str) -> Option<(&str, &str)>` that splits `"key = value"` into trimmed `("key", "value")`.
6. In `main`, collect several tokens into a `Vec<&str>` *while still holding the parser*, proving the tokens are not tied to the parser borrow.

## Expected behavior

- `longest("apple", "fig")` => `"apple"`
- `first_word("  hello world")` => `"hello"`
- `longest_with_prefix(&["car", "cartoon", "dog"], "car")` => `Some("cartoon")`
- `Parser::new("let x = 5")` yields `"let"`, `"x"`, `"="`, `"5"`, then `None`
- `split_key_value("name = Ferris")` => `Some(("name", "Ferris"))`
- `split_key_value("no equals")` => `None`

## Optional hints

- Elision rule: with exactly one reference input, the output gets that input's lifetime.
- `str::find(char::is_whitespace)` and `str::trim_start` are useful for the parser.
- Track `pos` as a byte offset into `input`; slice with `&self.input[start..end]`.
- `str::split_once('=')` returns `Option<(&str, &str)>`.

## Bonus challenge

Implement `Iterator for Parser<'a>` with `type Item = &'a str;` and use it in a `for` loop.
