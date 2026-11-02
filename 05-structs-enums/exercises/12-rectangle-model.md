# Exercise 12 — Rectangle Model

- **Title:** Rectangle Model
- **Objective:** Add behavior to a `Rectangle` struct using methods and associated functions.
- **Difficulty:** Beginner
- **Concepts practiced:** methods, associated functions, `#[derive(Debug)]`, borrowing `&self` and `&Other`

## Instructions

Implement on `Rectangle`:

1. `square(size: f64) -> Rectangle` (associated function).
2. `area(&self) -> f64` and `perimeter(&self) -> f64`.
3. `can_hold(&self, other: &Rectangle) -> bool` — `true` if `other` fits strictly inside `self`.
4. `scale(&mut self, factor: f64)` multiplies both sides.

## Expected behavior

```text
area: 50
perimeter: 30
can hold square: true
scaled: Rectangle { width: 20.0, height: 10.0 }
```

## Optional hints

- Add `#[derive(Debug)]` to print with `{:?}`.

## Bonus challenge

Add `is_square(&self) -> bool`.
