# Level 12 — Testing

## What you will learn
Unit tests, integration tests, `#[test]`, assertions, `cargo test`, test organization.

## Simple explanation
Tests lock expected behavior and make refactoring safer.

## Small syntax examples
```rust
#[test]
fn adds_two_numbers() { assert_eq!(2 + 2, 4); }
```

## Common mistakes
Testing implementation details, weak assertion messages, slow test setup.

## Exercises
Write tests for earlier modules and improve failing behavior.

## Hints
Prefer deterministic tests with clear names.

## Bonus challenges
Use table-driven patterns and error-case tests.

## References
<https://doc.rust-lang.org/book/ch11-00-testing.html>
