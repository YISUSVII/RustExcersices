# Level 16 — Macros

## What you will learn
`macro_rules!`, declarative macros, standard macros, procedural macro basics.

## Simple explanation
Macros generate Rust code, reducing repetitive patterns.

## Small syntax examples
```rust
macro_rules! say_hi { () => { println!("hi"); } }
```

## Common mistakes
Overengineering macros and unclear expansion behavior.

## Exercises
Create simple declarative macros for repetitive code patterns.

## Hints
Start with pattern matching tokens and small expansions.

## Bonus challenges
Inspect macro expansion with tooling.

## References
<https://doc.rust-lang.org/book/ch19-06-macros.html>
