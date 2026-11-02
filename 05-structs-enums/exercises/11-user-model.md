# Exercise 11 — User Model

- **Title:** User Model
- **Objective:** Model a user with a struct, an associated constructor, and methods.
- **Difficulty:** Beginner
- **Concepts practiced:** structs, `impl` blocks, associated functions, `&self` / `&mut self`

## Instructions

Complete the `User` struct in `11-user-model.rs`:

1. `User::new(username: &str, email: &str, age: u8) -> User` creates an active user.
2. `display_name(&self) -> String` returns `"username <email>"`.
3. `is_adult(&self) -> bool` returns `true` when `age >= 18`.
4. `deactivate(&mut self)` sets `active` to `false`.

## Expected behavior

```text
alice <alice@example.com>
adult: true
active: false
```

## Optional hints

- Use field init shorthand: `User { username, email, age, active: true }`.
- Convert `&str` to `String` with `.to_string()` or `String::from`.

## Bonus challenge

Use struct update syntax (`..user`) to create a second user that copies everything except the email.
