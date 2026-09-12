# Project — TCP Chat Server

- **Slug:** `p15-tcp-chat`
- **Difficulty:** Advanced
- **Objective:** Multi-client chat over TCP.

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Accept connections
2. Broadcast messages
3. Usernames

## Architecture hints

tokio or std threads + channels.

## Acceptance criteria

- Two clients can exchange messages

## Bonus challenges

- Rooms
- History

## Suggested layout

```text
p15-tcp-chat/
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
