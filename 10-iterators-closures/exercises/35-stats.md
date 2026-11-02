# Exercise 35 — Stats

- **Title:** Stats
- **Objective:** Compute descriptive statistics (mean, median, mode, variance, min/max) using iterators and `fold`, returning `Option`/`Result` instead of panicking on empty input.
- **Difficulty:** Intermediate
- **Concepts practiced:** `fold`, `sum`, `copied`, `HashMap` counting, sorting floats safely (`total_cmp`), `Option` for empty input, structs as return values

## Instructions

1. Implement `mean(data: &[f64]) -> Option<f64>`.
2. Implement `median(data: &[f64]) -> Option<f64>` — sort a copy (don't mutate the input); average the two middle values for even lengths.
3. Implement `mode(data: &[i64]) -> Option<i64>` — most frequent value; on ties return the **smallest** value.
4. Implement `variance(data: &[f64]) -> Option<f64>` (population variance) and `std_dev(data: &[f64]) -> Option<f64>`.
5. Implement `min_max(data: &[f64]) -> Option<(f64, f64)>` using a single `fold`.
6. Define
   ```rust
   #[derive(Debug, PartialEq)]
   struct Summary { count: usize, mean: f64, median: f64, min: f64, max: f64, std_dev: f64 }
   ```
   and `summarize(data: &[f64]) -> Option<Summary>` that reuses the functions above.
7. Implement `parse_data(input: &str) -> Result<Vec<f64>, String>` that parses comma-separated numbers (whitespace allowed) and rejects `NaN`.

## Expected behavior

- `mean(&[1.0, 2.0, 3.0, 4.0])` => `Some(2.5)`
- `mean(&[])` => `None`
- `median(&[3.0, 1.0, 2.0])` => `Some(2.0)`
- `median(&[4.0, 1.0, 3.0, 2.0])` => `Some(2.5)`
- `mode(&[1, 2, 2, 3, 3])` => `Some(2)`
- `variance(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0])` => `Some(4.0)`, `std_dev` => `Some(2.0)`
- `parse_data("1, 2.5,x")` => `Err("invalid number: 'x'")`

## Optional hints

- `let mut sorted = data.to_vec(); sorted.sort_by(f64::total_cmp);`
- For `mode`, count into a `HashMap<i64, usize>`, then `max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0)))`.
- `fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &x| (lo.min(x), hi.max(x)))`.

## Bonus challenge

Write `percentile(data: &[f64], p: f64) -> Option<f64>` using linear interpolation (p in `0.0..=100.0`, `None` otherwise).
