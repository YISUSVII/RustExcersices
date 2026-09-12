# Exercise 52 — Associated Type Container

- **Title:** Associated Types
- **Objective:** Use associated types in a trait.
- **Difficulty:** Ninja
- **Concepts practiced:** associated types, trait impls

## Instructions

1. Define `trait Container { type Item; fn first(&self) -> Option<&Self::Item>; }`.
2. Implement for `Vec<T>`.
3. Write `fn show_first<C: Container>(c: &C) where C::Item: std::fmt::Display`.

## Expected behavior

- `show_first` prints the first element of a `Vec`.

## Optional hints

- `type Item = T` inside the impl.

## Bonus challenge

Implement `Container` for `[T; N]`.
