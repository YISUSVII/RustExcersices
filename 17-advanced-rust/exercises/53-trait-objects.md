# Exercise 53 — Trait Object Catalog

- **Title:** Trait Objects
- **Objective:** Store heterogeneous values behind `dyn Trait`.
- **Difficulty:** Ninja
- **Concepts practiced:** trait objects, dynamic dispatch

## Instructions

1. Define `trait Describable { fn describe(&self) -> String; }`.
2. Implement for `i32` and `String`.
3. Implement `fn catalog(items: &[Box<dyn Describable>]) -> Vec<String>`.

## Expected behavior

- Mixed boxed values produce descriptions in order.

## Optional hints

- `Box<dyn Describable>` requires trait object safety (no generics on methods).

## Bonus challenge

Use `&[&dyn Describable]` instead of boxed owned values.
