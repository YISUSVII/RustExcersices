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
- [19 — Word Frequency](exercises/19-word-frequency.md)
- [20 — Grade Tracker](exercises/20-grade-tracker.md)
- [21 — Duplicate Remover](exercises/21-duplicate-remover.md)
- [22 — Inventory](exercises/22-inventory.md)
- [23 — Text Analyzer](exercises/23-text-analyzer.md)

See `exercises/` for starters and `../solutions/07-collections/` for reference solutions.

## Hints
Use iterators and entry APIs (`entry`, `or_insert`).

## Bonus challenges
Optimize memory allocations and avoid redundant clones.

## References
<https://doc.rust-lang.org/book/ch08-00-common-collections.html>
