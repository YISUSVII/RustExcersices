# Exercise 45 — Channel Pipeline

- **Title:** Producer / Consumer
- **Objective:** Move data between threads with channels.
- **Difficulty:** Advanced
- **Concepts practiced:** `mpsc`, producer/consumer

## Instructions

Implement `fn square_pipeline(inputs: Vec<i32>) -> Vec<i32>`:
- producer thread sends each input
- consumer collects squares into a `Vec` preserving order of receipt (same as input order if single consumer sequential)

## Expected behavior

- `[1, 2, 3]` => `[1, 4, 9]`

## Optional hints

- `mpsc::channel`, drop sender to end iteration.

## Bonus challenge

Use a bounded channel and multiple workers.
