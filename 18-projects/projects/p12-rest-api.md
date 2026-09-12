# Project — REST API

- **Slug:** `p12-rest-api`
- **Difficulty:** Advanced
- **Objective:** CRUD HTTP JSON API for a resource (e.g., notes).

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Routing
2. JSON encode/decode
3. In-memory store

## Architecture hints

Framework optional (axum/actix) or std + manual.

## Acceptance criteria

- POST/GET/DELETE behave RESTfully

## Bonus challenges

- Persistence
- Auth middleware

## Suggested layout

```text
p12-rest-api/
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
