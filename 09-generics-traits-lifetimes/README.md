# Level 9 — Generics, Traits and Lifetimes

## What you will learn
Generic functions/structs, traits, trait bounds, `where`, `impl Trait`, lifetimes.

## Simple explanation
Generics remove duplication, traits describe shared behavior, and lifetimes explain reference validity.

## Small syntax examples
```rust
fn max<T: Ord>(a: T, b: T) -> T { if a > b { a } else { b } }
```

## Common mistakes
Overconstraining bounds and returning references without valid lifetime relationships.

## Exercises
Generic max, printable trait, shape trait, repository abstraction, reference-returning APIs.

## Hints
Write constraints only when needed.

## Bonus challenges
Combine multiple trait bounds with `where`.

## References
<https://doc.rust-lang.org/book/ch10-00-generics.html>
