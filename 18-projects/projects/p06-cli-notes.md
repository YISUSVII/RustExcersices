# Project — CLI Notes Application

- **Slug:** `p06-cli-notes`
- **Difficulty:** Intermediate
- **Objective:** Create, list, search, and delete plain-text notes.

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Notes directory management
2. Search by substring
3. Delete by id/name

## Architecture hints

One file per note or an index JSON.

## Acceptance criteria

- CRUD operations work without crashing on missing notes

## Bonus challenges

- Full-text ranking
- Tags

## Suggested layout

```text
p06-cli-notes/
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
