# Level 17 — Advanced Rust

## What you will learn

Advanced traits, associated types, trait objects, dynamic dispatch, unsafe Rust, raw pointers, FFI concepts, pinning basics.

## Simple explanation

Advanced Rust balances zero-cost abstractions with controlled low-level power when needed.

## Small syntax examples

```rust
trait IteratorLike { type Item; }
```

## Common mistakes

Using `unsafe` without clear invariants and poor encapsulation.

## Exercises

| # | Exercise | Starter |
| - | -------- | ------- |
| 52 | Associated type container | `exercises/52-associated-type.rs` |
| 53 | Trait objects catalog | `exercises/53-trait-objects.rs` |
| 54 | Safe wrapper over unsafe | `exercises/54-safe-wrapper.rs` |

## Hints

Document invariants around unsafe blocks.

## Bonus challenges

Wrap unsafe internals in safe public APIs.

## References

- <https://doc.rust-lang.org/book/ch19-00-advanced-features.html>
