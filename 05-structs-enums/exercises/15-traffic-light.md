# Exercise 15 — Traffic Light Enum

- **Title:** Traffic Light
- **Objective:** Model exclusive states with an enum and transition method.
- **Difficulty:** Intermediate
- **Concepts practiced:** enums, `match`, methods

## Instructions

1. Define `enum TrafficLight { Red, Yellow, Green }`.
2. Implement `next(self) -> TrafficLight` cycling Green → Yellow → Red → Green.
3. Implement `fn label(&self) -> &'static str`.

## Expected behavior

- `TrafficLight::Green.next()` is `Yellow`.
- Labels are `"red"`, `"yellow"`, `"green"`.

## Optional hints

- Consume `self` in `next` or take `&self` and return a new value.

## Bonus challenge

Add a timer duration method per variant.
