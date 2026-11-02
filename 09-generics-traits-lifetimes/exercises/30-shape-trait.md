# Exercise 30 — Shape Trait

- **Title:** Shape Trait
- **Objective:** Model geometric shapes behind a common trait and compute aggregate values over a heterogeneous collection.
- **Difficulty:** Intermediate
- **Concepts practiced:** traits, trait objects (`Box<dyn Trait>`), generic functions with `where` clauses, supertraits, `Result` for validation

## Instructions

1. Define the trait:
   ```rust
   trait Shape: std::fmt::Debug {
       fn name(&self) -> &'static str;
       fn area(&self) -> f64;
       fn perimeter(&self) -> f64;
   }
   ```
2. Create `Circle { radius: f64 }`, `Rectangle { width: f64, height: f64 }` and `Triangle { a: f64, b: f64, c: f64 }`, each deriving `Debug` and implementing `Shape`.
   - Use Heron's formula for the triangle area.
3. Add validating constructors that return `Result<Self, String>`:
   - `Circle::new(radius: f64)` — radius must be `> 0`.
   - `Rectangle::new(width: f64, height: f64)` — both sides `> 0`.
   - `Triangle::new(a: f64, b: f64, c: f64)` — sides `> 0` and satisfy the triangle inequality.
4. Implement:
   - `total_area(shapes: &[Box<dyn Shape>]) -> f64`
   - `largest_shape(shapes: &[Box<dyn Shape>]) -> Option<&dyn Shape>` (by area)
   - `describe<S>(shape: &S) -> String where S: Shape + ?Sized` returning e.g. `"Circle: area=3.14, perimeter=6.28"` (2 decimals).
5. Build a `Vec<Box<dyn Shape>>` in `main` and print each description, the total area and the largest shape.

## Expected behavior

- `Circle::new(1.0)?.area()` ≈ `3.14159`
- `Rectangle::new(2.0, 3.0)?.perimeter()` => `10.0`
- `Triangle::new(3.0, 4.0, 5.0)?.area()` => `6.0`
- `Triangle::new(1.0, 2.0, 10.0)` => `Err("...triangle inequality...")`
- `Circle::new(-1.0)` => `Err(...)`

## Optional hints

- `std::f64::consts::PI`
- Heron: `s = (a + b + c) / 2`, `area = sqrt(s(s-a)(s-b)(s-c))`.
- Compare floats with `partial_cmp` or `f64::total_cmp` when finding a maximum.
- `?Sized` lets `describe` accept `dyn Shape` as well as concrete types.

## Bonus challenge

Add a `Square` that reuses `Rectangle` internally, and sort the shape list by area (descending) using `sort_by`.
