# Exercise 17 — Option Command Parser

- **Title:** Option / Result Command Handling
- **Objective:** Branch on `Option` and `Result` with `match` / `if let`.
- **Difficulty:** Intermediate
- **Concepts practiced:** `Option`, `Result`, `match`

## Instructions

Implement `fn run(cmd: Option<&str>) -> Result<&'static str, String>`:
- `None` => `Err("missing command")`
- `Some("help")` => `Ok("usage: help|version")`
- `Some("version")` => `Ok("1.0.0")`
- anything else => `Err("unknown command")`

## Expected behavior

- `run(Some("help"))` => `Ok("usage: help|version")`
- `run(None)` => `Err(...)`

## Optional hints

- A single `match` is enough.

## Bonus challenge

Accept case-insensitive commands.
