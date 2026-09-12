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

| # | Exercise | Starter |
| - | -------- | ------- |
| 25 | Parse with `?` | `exercises/25-parse-question-mark.rs` |
| 26 | Validate user input | `exercises/26-validate-input.rs` |
| 27 | Custom error enum | `exercises/27-custom-error.rs` |
| 28 | Read config lines | `exercises/28-config-lines.rs` |

## Hints

Prefer `?` and meaningful error messages.

## Bonus challenges

Model domain-specific errors with enums.

## References

- <https://doc.rust-lang.org/book/ch09-00-error-handling.html>
