# Exercise 33 — Data Transforms

- **Title:** Data Transforms
- **Objective:** Replace manual loops with iterator chains to transform, combine and summarize data.
- **Difficulty:** Intermediate
- **Concepts practiced:** `iter` vs `into_iter`, `map`, `filter`, `filter_map`, `zip`, `enumerate`, `fold`, `collect` into different collections, closures capturing variables

## Instructions

Implement each function using iterator adaptors (no `for` loops):

1. `squares_of_evens(nums: &[i32]) -> Vec<i32>` — keep even numbers and square them.
2. `parse_numbers(inputs: &[&str]) -> Vec<i32>` — parse each string (after trimming), silently skipping invalid ones.
3. `try_parse_all(inputs: &[&str]) -> Result<Vec<i32>, String>` — like above but fail on the first invalid input with `"invalid number: '<input>'"`.
4. `dot_product(a: &[f64], b: &[f64]) -> Option<f64>` — `None` if lengths differ.
5. `running_total(nums: &[i32]) -> Vec<i32>` — prefix sums, e.g. `[1, 2, 3]` → `[1, 3, 6]`.
6. `index_of_max(nums: &[i32]) -> Option<usize>` — index of the first maximum.
7. `scale_by(nums: &[i32], factor: i32) -> Vec<i32>` — the closure must capture `factor`.
8. `make_adder(n: i32) -> impl Fn(i32) -> i32` — return a closure that captures `n` with `move`.
9. `char_frequencies(text: &str) -> BTreeMap<char, usize>` — count alphabetic chars (lowercased) using `fold`.

## Expected behavior

- `squares_of_evens(&[1, 2, 3, 4])` => `[4, 16]`
- `parse_numbers(&["1", " 2 ", "x", "3"])` => `[1, 2, 3]`
- `try_parse_all(&["1", "x"])` => `Err("invalid number: 'x'")`
- `dot_product(&[1.0, 2.0], &[3.0, 4.0])` => `Some(11.0)`
- `running_total(&[1, 2, 3])` => `[1, 3, 6]`
- `index_of_max(&[3, 9, 2, 9])` => `Some(1)`
- `make_adder(10)(5)` => `15`
- `char_frequencies("Hello")` => `{'e': 1, 'h': 1, 'l': 2, 'o': 1}`

## Optional hints

- `collect::<Result<Vec<_>, _>>()` stops at the first `Err`.
- `scan` is great for running totals.
- `enumerate` + `fold` (or `max_by` with reversed index comparison) can find the first max index.
- `*map.entry(c).or_insert(0) += 1;` inside a `fold` closure that owns the map.

## Bonus challenge

Write `compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C` and use it to build a pipeline.
