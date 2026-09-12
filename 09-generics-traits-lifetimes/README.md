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

| # | Exercise | Starter |
| - | -------- | ------- |
| 29 | Generic max | `exercises/29-generic-max.rs` |
| 30 | Printable trait | `exercises/30-printable-trait.rs` |
| 31 | Shape trait | `exercises/31-shape-trait.rs` |
| 32 | Longest lifetime | `exercises/32-longest-lifetime.rs` |

## Hints

Write constraints only when needed.

## Bonus challenges

Combine multiple trait bounds with `where`.

## References

- <https://doc.rust-lang.org/book/ch10-00-generics.html>
