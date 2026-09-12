# Project — Tiny Redis-like Server

- **Slug:** `p16-mini-redis`
- **Difficulty:** Ninja
- **Objective:** Subset of Redis protocol: PING, GET, SET, DEL.

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. RESP parsing
2. In-memory map
3. TCP server

## Architecture hints

Keep protocol correct for redis-cli basics if possible.

## Acceptance criteria

- PING/SET/GET work

## Bonus challenges

- EXPIRE
- Persistence AOF

## Suggested layout

```text
p16-mini-redis/
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
