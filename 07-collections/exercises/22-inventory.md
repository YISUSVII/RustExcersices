# Exercise 22 — Inventory

- **Title:** Inventory Manager
- **Objective:** Manage stock levels in a `HashMap` with safe updates and reports.
- **Difficulty:** Intermediate
- **Concepts practiced:** `HashMap<String, Item>`, `entry` API, `get_mut`, `Result` for invalid updates, iterator aggregation

## Instructions

Items are `Item { quantity: u32, price_cents: u64 }`. Implement `Inventory`:

1. `Inventory::new() -> Inventory`
2. `add(&mut self, name: &str, quantity: u32, price_cents: u64)` — insert a new item or increase the quantity of an existing one (and update its price).
3. `remove(&mut self, name: &str, quantity: u32) -> Result<u32, String>` — returns the remaining quantity; errors for unknown items or insufficient stock. Remove the entry when it reaches `0`.
4. `total_value(&self) -> u64` — sum of `quantity * price_cents`.
5. `low_stock(&self, threshold: u32) -> Vec<&str>` — names with `quantity < threshold`, sorted alphabetically.

## Expected behavior

```text
remove 3 apples -> Ok(7)
remove 5 kiwis -> Err("unknown item: kiwi")
remove 9 pears -> Err("not enough pear: have 4, requested 9")
remove 4 pears -> Ok(0)
total value: $9.80
low stock (< 8): ["apple", "banana"]
```

## Optional hints

- `let item = self.items.get_mut(name).ok_or_else(|| format!(...))?;`
- `low_stock` can return `&str` borrowed from the map keys — no clones needed.

## Bonus challenge

Format `total_value` as dollars with a `format_cents(cents: u64) -> String` helper.
