# Level 6 — Pattern Matching

## What you will learn

`match`, `if let`, `let else`, destructuring, wildcard patterns, guards.

## Simple explanation

Pattern matching lets you unpack and validate data in one clear step.

## Small syntax examples

```rust
if let Some(value) = maybe { println!("{value}"); }
```

## Common mistakes

Ignoring unmatched cases, overusing `_`, and missing guard conditions.

## Exercises

| # | Exercise | Starter |
| - | -------- | ------- |
| 17 | Option command parser | `exercises/17-option-commands.rs` |
| 18 | Enum destructuring | `exercises/18-enum-destructure.rs` |
| 19 | Match guards | `exercises/19-match-guards.rs` |

## Hints

Prefer explicit matches before compact syntax.

## Bonus challenges

Refactor nested `if` trees into a single `match`.

## References

- <https://doc.rust-lang.org/book/ch06-02-match.html>
