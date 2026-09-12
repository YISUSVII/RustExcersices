# Project — Multithreaded Web Server

- **Slug:** `p11-web-server`
- **Difficulty:** Advanced
- **Objective:** Serve static files over HTTP/1.1 with a thread pool.

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Parse requests
2. Thread pool
3. 404 handling

## Architecture hints

Follow Rust Book ch20 structure.

## Acceptance criteria

- GET / returns 200 for index
- Unknown paths 404

## Bonus challenges

- Connection keep-alive
- Logging

## Suggested layout

```text
p11-web-server/
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
