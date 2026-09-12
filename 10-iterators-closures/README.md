# Level 10 — Closures and Iterators

## What you will learn

Closures, Iterator trait, `map`, `filter`, `fold`, `collect`, `enumerate`, `zip`.

## Simple explanation

Iterators transform data lazily and compositionally; closures capture surrounding context.

## Small syntax examples

```rust
let sum: i32 = vec![1,2,3].into_iter().map(|n| n * 2).sum();
```

## Common mistakes

Collecting too early, ownership confusion in closures, unreadable chains.

## Exercises

| # | Exercise | Starter |
| - | -------- | ------- |
| 33 | Map and filter | `exercises/33-map-filter.rs` |
| 34 | User filtering | `exercises/34-user-filtering.rs` |
| 35 | Stats with fold | `exercises/35-stats-fold.rs` |
| 36 | Word processing | `exercises/36-word-processing.rs` |

## Hints

Build chains step by step, then refactor.

## Bonus challenges

Implement custom iterator adaptors.

## References

- <https://doc.rust-lang.org/book/ch13-00-functional-features.html>
