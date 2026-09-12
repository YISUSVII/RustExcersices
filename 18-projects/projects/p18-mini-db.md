# Project — Basic Database Engine

- **Slug:** `p18-mini-db`
- **Difficulty:** Ninja
- **Objective:** Tiny SQL-like engine: CREATE/INSERT/SELECT for one table.

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Storage format
2. Parser
3. Query execution

## Architecture hints

Row-oriented file or memory first.

## Acceptance criteria

- Round-trip insert/select works

## Bonus challenges

- Indexes
- UPDATE

## Suggested layout

```text
p18-mini-db/
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
