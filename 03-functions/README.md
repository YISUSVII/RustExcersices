# Level 3 — Functions

## What you will learn

- Function declarations
- Parameters and return values
- Expressions vs statements
- Scope and reuse

## Simple explanation

Functions help organize logic into reusable units. In Rust, the last expression of a block can be an implicit return value.

## Small syntax examples

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```rust
fn main() {
    let result = add(2, 3);
    println!("{result}");
}
```

## Common mistakes

- Adding `;` to the final expression unintentionally
- Returning the wrong type
- Overloading one function with too many responsibilities

## Exercises

| # | Exercise | Starter |
| - | -------- | ------- |
| 08 | Reusable calculator | `exercises/08-reusable-calculator.rs` |

## Hints

- Name functions by behavior (`calculate_area`, `to_celsius`).
- Keep each function doing one thing.

## Bonus challenges

- Write unit tests for pure functions.

## References

- <https://doc.rust-lang.org/book/ch03-03-how-functions-work.html>
