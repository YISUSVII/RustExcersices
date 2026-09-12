# Exercise 41 — Box Linked List

- **Title:** Box Linked List
- **Objective:** Build a simple cons list with `Box`.
- **Difficulty:** Advanced
- **Concepts practiced:** `Box`, recursive types

## Instructions

1. Define `enum List { Cons(i32, Box<List>), Nil }`.
2. Implement `fn sum(list: &List) -> i32`.
3. Implement `fn from_slice(items: &[i32]) -> List` building front-to-back or back-to-front consistently.

## Expected behavior

- `from_slice(&[1, 2, 3])` sums to `6`.

## Optional hints

- Recursive enum variants need `Box`.

## Bonus challenge

Implement `Display` for pretty printing.
