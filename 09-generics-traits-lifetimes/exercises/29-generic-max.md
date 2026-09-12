# Exercise 29 — Generic Max

- **Title:** Generic Max
- **Objective:** Write a generic function with trait bounds.
- **Difficulty:** Advanced
- **Concepts practiced:** generics, `Ord` / `PartialOrd`

## Instructions

Implement `fn max_of<T: Ord>(a: T, b: T) -> T`.

## Expected behavior

- `max_of(3, 10)` => `10`
- `max_of('a', 'z')` => `'z'`

## Optional hints

- Compare with `>=` or `cmp`.

## Bonus challenge

Implement `max_slice(items: &[T]) -> Option<&T>`.
