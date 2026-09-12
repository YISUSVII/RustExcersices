# Project — Async HTTP Client

- **Slug:** `p13-async-http-client`
- **Difficulty:** Advanced
- **Objective:** Fetch multiple URLs concurrently and report status/body length.

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Tokio runtime
2. Concurrent requests
3. Timeouts

## Architecture hints

reqwest + tokio.

## Acceptance criteria

- Handles success and failure per URL

## Bonus challenges

- Retries with backoff

## Suggested layout

```text
p13-async-http-client/
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
