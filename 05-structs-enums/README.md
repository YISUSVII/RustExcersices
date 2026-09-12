# Level 5 — Structs and Enums

## What you will learn
Structs, tuple structs, methods, associated functions, enums, `Option`, `Result`.

## Simple explanation
Structs model data; enums model a value that can be one of multiple variants.

## Small syntax examples
```rust
struct User { name: String, age: u8 }
enum TrafficLight { Red, Yellow, Green }
```

## Common mistakes
Forgetting `impl` blocks, overusing mutable state, not matching all enum variants.

## Exercises
User model, rectangle model, bank account, state machine, traffic light enum.

## Hints
Use methods to keep behavior near data.

## Bonus challenges
Implement validation logic with `Result`.

## References
<https://doc.rust-lang.org/book/ch05-00-structs.html>
<https://doc.rust-lang.org/book/ch06-00-enums.html>
