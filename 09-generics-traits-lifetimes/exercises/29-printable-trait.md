# Exercise 29 — Printable Trait

- **Title:** Printable Trait
- **Objective:** Define a custom trait with a required method and a default method, implement it for several types, and use it through generics, `impl Trait` and trait objects.
- **Difficulty:** Intermediate
- **Concepts practiced:** traits, default methods, trait bounds, `impl Trait`, `dyn Trait`, `std::fmt::Display`

## Instructions

1. Define a trait:
   ```rust
   trait Printable {
       fn label(&self) -> String;
       fn print(&self) -> String { /* default: "[" + label + "]" */ }
   }
   ```
2. Create two structs and implement `Printable` for them:
   - `Book { title: String, author: String }` → label `"Book: <title> by <author>"`
   - `Temperature { celsius: f64 }` → label `"Temp: <celsius>°C"` with 1 decimal place. **Override** `print` to return `"~<label>~"`.
3. Implement `Printable` for the built-in type `i32` → label `"Number: <n>"`.
4. Implement these functions:
   - `print_one<T: Printable>(item: &T) -> String` — static dispatch with a trait bound.
   - `print_all(items: &[&dyn Printable]) -> Vec<String>` — dynamic dispatch with trait objects.
   - `make_printable(n: i32) -> impl Printable` — return an opaque type.
5. Call everything from `main`.

## Expected behavior

- `Book { title: "Dune", author: "Herbert" }.print()` => `"[Book: Dune by Herbert]"`
- `Temperature { celsius: 21.456 }.print()` => `"~Temp: 21.5°C~"`
- `42.print()` => `"[Number: 42]"`
- `print_all(&[&book, &temp, &7])` returns three strings in order.

## Optional hints

- A default method can call required methods: `format!("[{}]", self.label())`.
- `&dyn Printable` needs the trait to be *object safe* (no generic methods, no `Self` return).
- You may implement your own trait for foreign types like `i32` (the orphan rule allows it because the trait is local).

## Bonus challenge

Add a blanket implementation: `impl<T: Printable> Printable for Vec<T>` whose label joins inner labels with `", "`.
