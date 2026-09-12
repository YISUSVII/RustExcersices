# Exercise 47 — Async Timer

- **Title:** Async Timer Lab
- **Objective:** Run delayed tasks on Tokio.
- **Difficulty:** Advanced
- **Concepts practiced:** `async fn`, `tokio::time`, `cargo` features

## Instructions

1. In `exercises/47-async-timer`, implement `async fn delayed_message(ms: u64, msg: &str) -> String` that sleeps then returns the message.
2. Call it from `#[tokio::main]`.
3. `cargo run` should print the message after the delay.

## Expected behavior

- `delayed_message(10, "done").await` => `"done"`

## Optional hints

- `tokio::time::sleep(Duration::from_millis(ms)).await`.

## Bonus challenge

Race two timers with `tokio::select!`.
