# Level 11 — Modules, Crates and Project Structure

## What you will learn
`mod`, `pub`, `use`, crate layout, library vs binary, Cargo.toml, dependencies, workspaces.

## Simple explanation
Modules organize code; crates package code for reuse; workspaces manage multiple crates.

## Small syntax examples
```rust
mod math;
use math::add;
```

## Common mistakes
Incorrect module paths, missing `pub`, circular structure confusion.

## Exercises
- [37 — Multi-Module Project](exercises/37-multi-module-project.md)

See `exercises/` for starters and `../solutions/11-modules-crates/` for reference solutions.

## Hints
Start with a library crate, then add a binary crate.

## Bonus challenges
Create a mini workspace with shared utilities.

## References
<https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html>
