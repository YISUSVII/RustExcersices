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
- [16 — Option/Result Branching](exercises/16-option-result-branching.md)
- [17 — Enum Destructuring](exercises/17-enum-destructuring.md)
- [18 — Nested `if` to `match`](exercises/18-nested-if-to-match.md)

See `exercises/` for starters and `../solutions/06-pattern-matching/` for reference solutions.

## Hints
Prefer explicit matches before compact syntax.

## Bonus challenges
Refactor nested `if` trees into a single `match`.

## References
<https://doc.rust-lang.org/book/ch06-02-match.html>
