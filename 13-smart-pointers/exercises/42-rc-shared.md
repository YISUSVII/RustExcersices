# Exercise 42 — Rc Shared Graph Node

- **Title:** Rc Shared Node
- **Objective:** Share ownership of a value with `Rc`.
- **Difficulty:** Advanced
- **Concepts practiced:** `Rc`, `Rc::strong_count`

## Instructions

1. Create a shared `Rc<String>` label.
2. Implement `fn attach_label(base: &Rc<String>) -> Rc<String>` cloning the `Rc` (not the string).
3. Print strong counts before/after.

## Expected behavior

- After one attach, strong count is at least 2.

## Optional hints

- `Rc::clone(base)` bumps the count.

## Bonus challenge

Use `Weak` to observe without owning.
