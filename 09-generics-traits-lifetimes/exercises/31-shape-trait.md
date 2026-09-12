# Exercise 31 — Shape Trait

- **Title:** Shape Trait
- **Objective:** Share behavior across types with a trait object-friendly API.
- **Difficulty:** Advanced
- **Concepts practiced:** traits, generics over trait bounds

## Instructions

1. Define `trait Shape { fn area(&self) -> f64; }`.
2. Implement for `Circle { radius: f64 }` and `Rectangle { w: f64, h: f64 }`.
3. Implement `fn total_area(shapes: &[impl Shape])` is awkward for mixed types — instead implement `fn total_area(shapes: &[&dyn Shape]) -> f64`.

## Expected behavior

- Circle r=1 area ≈ `3.14159...`
- Sum of circle r=1 and 2x3 rectangle ≈ `π + 6`

## Optional hints

- Use `std::f64::consts::PI`.

## Bonus challenge

Add `fn name(&self) -> &'static str` to the trait.
