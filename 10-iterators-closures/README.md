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
- [33 — Data Transforms](exercises/33-data-transforms.md)
- [34 — User Filtering](exercises/34-user-filtering.md)
- [35 — Stats](exercises/35-stats.md)
- [36 — Word Processing](exercises/36-word-processing.md)

See `exercises/` for starters and `../solutions/10-iterators-closures/` for reference solutions.

## Hints
Build chains step by step, then refactor.

## Bonus challenges
Implement custom iterator adaptors.

## References
<https://doc.rust-lang.org/book/ch13-00-functional-features.html>
