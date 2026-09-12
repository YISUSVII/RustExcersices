# Project — Number Guessing Game

- **Slug:** `p02-guessing-game`
- **Difficulty:** Beginner
- **Objective:** Player guesses a secret number with high/low feedback.

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Generate secret in range
2. Read guesses
3. Loop until correct

## Architecture hints

Use std input; optional rand crate.

## Acceptance criteria

- Game exits on correct guess with attempt count

## Bonus challenges

- Difficulty levels
- Persist best score

## Suggested layout

```text
p02-guessing-game/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── lib.rs
└── tests/
```

## Progress

- [ ] Milestone 1
- [ ] Milestone 2
- [ ] Milestone 3
- [ ] Acceptance criteria met
- [ ] Bonus (optional)
