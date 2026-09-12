# Project — Mini Grep

- **Slug:** `p10-mini-grep`
- **Difficulty:** Intermediate
- **Objective:** Search file contents for a query string.

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. CLI: query + path
2. Print matching lines with numbers
3. Case-insensitive flag

## Architecture hints

Inspired by Rust Book minigrep.

## Acceptance criteria

- Finds known lines in fixture

## Bonus challenges

- Regex support

## Suggested layout

```text
p10-mini-grep/
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
