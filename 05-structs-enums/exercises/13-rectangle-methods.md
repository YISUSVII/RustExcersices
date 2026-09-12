# Exercise 13 — Rectangle Methods

- **Title:** Rectangle with Methods
- **Objective:** Attach area/perimeter behavior to a struct.
- **Difficulty:** Intermediate
- **Concepts practiced:** methods, `&self`

## Instructions

1. Define `struct Rectangle { width: f64, height: f64 }`.
2. Implement `area(&self) -> f64` and `perimeter(&self) -> f64`.
3. Implement `can_hold(&self, other: &Rectangle) -> bool`.

## Expected behavior

- `Rectangle { width: 10.0, height: 5.0 }.area()` => `50.0`
- A 10x5 rectangle can hold a 4x3 rectangle.

## Optional hints

- Perimeter is `2.0 * (width + height)`.

## Bonus challenge

Add `square(size: f64) -> Rectangle` associated function.
