# Level 1 — Rust Fundamentals

## What you will learn

- Variables with `let`
- Mutability, constants, and shadowing
- Primitive types: integers, floats, booleans, chars
- Compound types: tuples and arrays

## Simple explanation

Rust variables are immutable by default. You explicitly opt into mutability with `mut`. Rust's type system helps catch mistakes early and encourages explicit intent.

## Small syntax examples

```rust
let x = 10;
let mut y = 5;
y += 1;
const MAX_USERS: u32 = 100;
let y = y * 2; // shadowing
```

```rust
let t: (i32, f64, char) = (42, 3.14, 'R');
let arr = [1, 2, 3, 4];
```

## Common mistakes

- Trying to mutate a non-`mut` variable
- Confusing shadowing with mutation
- Using the wrong numeric type for calculations

## Exercises

| # | Exercise | Starter |
| - | -------- | ------- |
| 01 | Temperature converter | `exercises/01-temperature-converter.rs` |
| 02 | Rectangle area | `exercises/02-rectangle-area.rs` |
| 03 | Basic calculator | `exercises/03-basic-calculator.rs` |
| 04 | Unit converter | `exercises/04-unit-converter.rs` |

## Hints

- Parse input safely and return useful errors.
- Keep functions small and focused.

## Bonus challenges

- Add unit conversion menus.
- Support decimal inputs and better formatting.

## References

- <https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html>
- <https://doc.rust-lang.org/std/primitive/>
