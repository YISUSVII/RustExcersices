# Level 7 — Collections and Strings

## What you will learn

`Vec<T>`, `String`, `HashMap`, `HashSet`, iteration, updates, ownership in collections.

## Simple explanation

Collections store multiple values and often own their contents, so ownership rules still apply.

## Small syntax examples

```rust
use std::collections::HashMap;
let mut counts = HashMap::new();
counts.insert("rust", 1);
```

## Common mistakes

Index out of bounds, cloning unnecessarily, misunderstanding borrowed keys.

## Exercises

| # | Exercise | Starter |
| - | -------- | ------- |
| 20 | Word frequency | `exercises/20-word-frequency.rs` |
| 21 | Grade tracker | `exercises/21-grade-tracker.rs` |
| 22 | Duplicate remover | `exercises/22-duplicate-remover.rs` |
| 23 | Inventory | `exercises/23-inventory.rs` |
| 24 | Text analyzer | `exercises/24-text-analyzer.rs` |

## Hints

Use iterators and entry APIs (`entry`, `or_insert`).

## Bonus challenges

Optimize memory allocations and avoid redundant clones.

## References

- <https://doc.rust-lang.org/book/ch08-00-common-collections.html>
