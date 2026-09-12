# Exercise 23 — Inventory

- **Title:** Inventory Manager
- **Objective:** Update stock quantities safely.
- **Difficulty:** Intermediate
- **Concepts practiced:** `HashMap`, mutable updates

## Instructions

Implement:
- `fn stock_in(inv: &mut HashMap<String, i32>, sku: &str, qty: i32)`
- `fn stock_out(inv: &mut HashMap<String, i32>, sku: &str, qty: i32) -> Result<(), String>`
- `fn quantity(inv: &HashMap<String, i32>, sku: &str) -> i32` (missing => 0)

`stock_out` fails when quantity would go negative.

## Expected behavior

- In 5, out 2 => 3 remaining; out 10 => error.

## Optional hints

- Insert missing SKUs as 0 on stock_in.

## Bonus challenge

Prevent non-positive qty with validation.
