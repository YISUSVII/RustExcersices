# Exercise 16 — Simple State Machine

- **Title:** Door State Machine
- **Objective:** Encode valid transitions with enums and `Result`.
- **Difficulty:** Intermediate
- **Concepts practiced:** enums, exhaustive match, domain errors

## Instructions

1. Define `enum Door { Closed, Open, Locked }`.
2. Implement:
   - `open(self) -> Result<Door, String>`
   - `close(self) -> Result<Door, String>`
   - `lock(self) -> Result<Door, String>`
   - `unlock(self) -> Result<Door, String>`
3. Rules: only Closed can open or lock; only Open can close; only Locked can unlock.

## Expected behavior

- `Closed.open()` => `Ok(Open)`
- `Locked.open()` => `Err(...)`

## Optional hints

- Match on `self` and return `Err` for illegal transitions.

## Bonus challenge

Add an `Event` enum and a single `transition(self, event) -> Result<Door, String>`.
