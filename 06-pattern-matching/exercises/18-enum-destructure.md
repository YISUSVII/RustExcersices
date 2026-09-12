# Exercise 18 — Enum Destructuring

- **Title:** Enum Destructuring
- **Objective:** Extract data from enum variants with patterns.
- **Difficulty:** Intermediate
- **Concepts practiced:** destructuring, tuple/struct variants

## Instructions

1. Define `enum Message { Quit, Move { x: i32, y: i32 }, Write(String), ChangeColor(u8, u8, u8) }`.
2. Implement `fn describe(msg: &Message) -> String` using `match`.

## Expected behavior

- `Message::Move { x: 2, y: 3 }` describes both coordinates.
- `Message::ChangeColor(1, 2, 3)` includes all three channels.

## Optional hints

- Use `ref` or match on `&Message` to avoid moving `String`.

## Bonus challenge

Implement `fn translate(msg: Message, dx: i32, dy: i32) -> Message` shifting `Move` only.
