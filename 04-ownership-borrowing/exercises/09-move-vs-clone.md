# Exercise 09 — Move vs Clone (Broken Code)

- **Title:** Move vs Clone
- **Objective:** Understand ownership moves and fix compiler error `E0382`.
- **Difficulty:** Intermediate
- **Concepts practiced:** ownership, move semantics, cloning

## Instructions

The starter code intentionally fails to compile because it tries to use a moved value.
Fix it in one of these ways:
1. Borrow the value where possible
2. Clone only when needed

## Expected behavior

Program prints the original message at least once and does not use invalid moved values.

## Optional hints

- Function parameters of type `String` take ownership.
- `&str` or `&String` can avoid moves.

## Bonus challenge

Refactor to avoid any clone at all.
