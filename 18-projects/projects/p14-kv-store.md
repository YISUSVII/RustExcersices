# Project — In-memory Key Value Store

- **Slug:** `p14-kv-store`
- **Difficulty:** Advanced
- **Objective:** Map-like store with get/set/del and optional TTL.

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Thread-safe store
2. CLI or TCP interface
3. Basic commands

## Architecture hints

Mutex/RwLock or sharded maps.

## Acceptance criteria

- Commands satisfy acceptance script

## Bonus challenges

- Snapshots to disk

## Suggested layout

```text
p14-kv-store/
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
