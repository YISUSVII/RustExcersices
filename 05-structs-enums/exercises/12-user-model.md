# Exercise 12 — User Model

- **Title:** User Model
- **Objective:** Define a struct and associated constructors/methods.
- **Difficulty:** Intermediate
- **Concepts practiced:** structs, `impl`, associated functions

## Instructions

1. Define `struct User { name: String, age: u8, active: bool }`.
2. Implement `User::new(name: String, age: u8) -> User` with `active: true`.
3. Implement `fn display(&self) -> String` returning a readable summary.
4. In `main`, create a user and print the display string.

## Expected behavior

- `User::new("Ada".into(), 36).display()` contains name and age.

## Optional hints

- Use `format!` inside `display`.

## Bonus challenge

Return `Result<User, String>` from `new` when `age == 0` or name is empty.
