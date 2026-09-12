# Project — Temperature Converter CLI

- **Slug:** `p04-temperature-converter`
- **Difficulty:** Beginner
- **Objective:** Convert C/F/K from CLI flags or prompt.

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Parse unit + value
2. Convert accurately
3. Validate absolute zero

## Architecture hints

Enum Unit + match conversions.

## Acceptance criteria

- Known sample conversions match expected values

## Bonus challenges

- Batch convert CSV

## Suggested layout

```text
p04-temperature-converter/
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
