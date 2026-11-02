# Level 8 — Error Handling

## What you will learn
`panic!`, `Result`, `Option`, `unwrap`, `expect`, `?`, custom errors.

## Simple explanation
Rust distinguishes recoverable errors (`Result`) from unrecoverable failures (`panic!`).

## Small syntax examples
```rust
fn parse_num(s: &str) -> Result<i32, std::num::ParseIntError> { s.parse() }
```

## Common mistakes
Overusing `unwrap`, swallowing context, and returning vague error types.

## Exercises
- [24 — File Reading](exercises/24-file-reading.md)
- [25 — Parsing](exercises/25-parsing.md)
- [26 — Data Validation](exercises/26-data-validation.md)
- [27 — Custom Errors](exercises/27-custom-errors.md)

See `exercises/` for starters and `../solutions/08-error-handling/` for reference solutions.

## Hints
Prefer `?` and meaningful error messages.

## Bonus challenges
Model domain-specific errors with enums.

## References
<https://doc.rust-lang.org/book/ch09-00-error-handling.html>
