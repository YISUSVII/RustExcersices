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
- [28 — Generic Max](exercises/28-generic-max.md)
- [29 — Printable Trait](exercises/29-printable-trait.md)
- [30 — Shape Trait](exercises/30-shape-trait.md)
- [31 — Repository Abstraction](exercises/31-repository-abstraction.md)
- [32 — Reference-Returning APIs](exercises/32-reference-returning-apis.md)

See `exercises/` for starters and `../solutions/09-generics-traits-lifetimes/` for reference solutions.

## Hints
Write constraints only when needed.

## Bonus challenges
Combine multiple trait bounds with `where`.

## References
<https://doc.rust-lang.org/book/ch10-00-generics.html>
