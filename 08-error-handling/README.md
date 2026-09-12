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
File reading, parsing, data validation, custom error design.

## Hints
Prefer `?` and meaningful error messages.

## Bonus challenges
Model domain-specific errors with enums.

## References
<https://doc.rust-lang.org/book/ch09-00-error-handling.html>
