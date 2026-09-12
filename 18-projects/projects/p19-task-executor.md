# Project — Multithreaded Task Executor

- **Slug:** `p19-task-executor`
- **Difficulty:** Ninja
- **Objective:** Work-stealing or simple thread-pool executor for closures/jobs.

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Submit jobs
2. Shutdown
3. Error propagation

## Architecture hints

channels + workers.

## Acceptance criteria

- N jobs complete exactly once

## Bonus challenges

- Priorities
- Work stealing

## Suggested layout

```text
p19-task-executor/
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
