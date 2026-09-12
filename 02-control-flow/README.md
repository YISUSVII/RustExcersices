# Level 2 — Control Flow

## What you will learn

- `if` / `else`
- `match`
- `loop`, `while`, `for`
- ranges and loop control

## Simple explanation

Control flow lets your program decide and repeat behavior. Rust's `match` is exhaustive, which means the compiler forces you to handle all possible cases.

## Small syntax examples

```rust
if n % 2 == 0 { "even" } else { "odd" }
```

```rust
for i in 1..=5 {
    println!("{i}");
}
```

## Common mistakes

- Infinite loops without `break`
- Forgetting `_` wildcard in `match`
- Off-by-one mistakes in ranges

## Exercises

| # | Exercise | Starter |
| - | -------- | ------- |
| 05 | FizzBuzz | `exercises/05-fizzbuzz.rs` |
| 06 | Fibonacci | `exercises/06-fibonacci.rs` |
| 07 | Prime checker | `exercises/07-prime-checker.rs` |

## Hints

- Start with basic loop logic first, then refactor.
- Write helper functions for readability.

## Bonus challenges

- Add input validation and graceful retry loops.

## References

- <https://doc.rust-lang.org/book/ch03-05-control-flow.html>
- <https://doc.rust-lang.org/std/keyword.match.html>
