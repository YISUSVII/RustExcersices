# Level 16 — Macros

## What you will learn

`macro_rules!`, declarative macros, standard macros, procedural macro basics.

## Simple explanation

Macros generate Rust code, reducing repetitive patterns.

## Small syntax examples

```rust
macro_rules! say_hi { () => { println!("hi"); } }
```

## Common mistakes

Overengineering macros and unclear expansion behavior.

## Exercises

| # | Exercise | Starter |
| - | -------- | ------- |
| 50 | `vec_of!` macro | `exercises/50-vec-of-macro.rs` |
| 51 | `hashmap!` macro | `exercises/51-hashmap-macro.rs` |

## Hints

Start with pattern matching tokens and small expansions.

## Bonus challenges

Inspect macro expansion with tooling.

## References

- <https://doc.rust-lang.org/book/ch19-06-macros.html>
