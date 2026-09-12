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
Data transforms, user filtering, stats, word processing.

## Hints
Build chains step by step, then refactor.

## Bonus challenges
Implement custom iterator adaptors.

## References
<https://doc.rust-lang.org/book/ch13-00-functional-features.html>
