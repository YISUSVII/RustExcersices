# Exercise 18 — Nested `if` to `match`

- **Title:** Refactor Nested Ifs into `match`
- **Objective:** Replace a deeply nested `if` tree with a flat `match` using guards and `let ... else`.
- **Difficulty:** Intermediate
- **Concepts practiced:** `let ... else`, matching on tuples, match guards, string-literal patterns, refactoring safely with tests

## Instructions

The starter contains `shipping_cost_legacy(order: &Order) -> Result<u32, &'static str>`, a working but hard-to-read nested `if` implementation.

1. Implement `shipping_cost(order: &Order) -> Result<u32, &'static str>` with **identical behavior**:
   - Use `let Some(weight) = order.weight_kg else { return Err(...) };` to handle the missing weight.
   - Use one `match (order.country, order.express, weight)` with guards for every remaining rule.
   - Do not use `if` inside your function body.
2. Implement `parse_point(input: &str) -> Option<(i32, i32)>` for strings like `"3,-4"` using `let ... else` for each step (split, parse x, parse y).
3. Run the tests: they check that both shipping implementations agree on every case.

## Expected behavior

```text
Order { weight_kg: Some(3), country: "US", express: false } => Ok(800)
Order { weight_kg: Some(5), country: "US", express: true } => Ok(2000)
Order { weight_kg: Some(2), country: "MX", express: true } => Err("no international express")
Order { weight_kg: None, country: "US", express: false } => Err("missing weight")
matches legacy: true
parse_point("3,-4") = Some((3, -4))
parse_point("oops") = None
```

## Optional hints

- Put the most specific arms first (e.g. `(_, _, 0)` before `("US", false, w)`).
- Guards look like `("US", true, w) if w > 20 => ...`.
- `str::split_once(',')` returns `Option<(&str, &str)>`.

## Bonus challenge

Replace the `&'static str` errors with a `ShippingError` enum.
