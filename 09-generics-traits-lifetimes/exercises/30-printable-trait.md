# Exercise 30 — Printable Trait

- **Title:** Printable Trait
- **Objective:** Define and implement a simple trait.
- **Difficulty:** Advanced
- **Concepts practiced:** traits, `impl Trait for T`

## Instructions

1. Define `trait Printable { fn brief(&self) -> String; }`.
2. Implement it for a `struct Book { title: String, pages: u32 }`.
3. Write `fn show(item: &impl Printable)` that prints `brief()`.

## Expected behavior

- A book titled `"Rust"` with 500 pages prints a summary including both fields.

## Optional hints

- `impl Printable for Book { ... }`.

## Bonus challenge

Implement `Printable` for `String` too.
