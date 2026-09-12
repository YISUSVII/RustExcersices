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
Option/Result-heavy branch handling and enum destructuring drills.

## Hints
Prefer explicit matches before compact syntax.

## Bonus challenges
Refactor nested `if` trees into a single `match`.

## References
<https://doc.rust-lang.org/book/ch06-02-match.html>
