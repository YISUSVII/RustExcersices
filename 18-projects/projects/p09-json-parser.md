# Project — JSON Parser Exercise

- **Slug:** `p09-json-parser`
- **Difficulty:** Intermediate
- **Objective:** Parse a restricted JSON subset (objects of string/number/bool/null/arrays simplified).

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Tokenize
2. Parse values
3. Error on invalid input

## Architecture hints

No full serde reimplementation required; educational subset.

## Acceptance criteria

- Parses provided fixtures
- Rejects invalid JSON with line info

## Bonus challenges

- Pretty printer

## Suggested layout

```text
p09-json-parser/
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
