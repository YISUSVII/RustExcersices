# Project — File Search Tool

- **Slug:** `p07-file-search`
- **Difficulty:** Intermediate
- **Objective:** Recursively find files by name pattern.

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Walk directories
2. Filter by glob/substring
3. Print paths

## Architecture hints

Avoid symlink loops; use std::fs.

## Acceptance criteria

- Finds nested matches
- Handles permission errors gracefully

## Bonus challenges

- Parallel walk
- Ignore globs

## Suggested layout

```text
p07-file-search/
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
