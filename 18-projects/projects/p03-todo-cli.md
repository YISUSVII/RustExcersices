# Project — Todo CLI

- **Slug:** `p03-todo-cli`
- **Difficulty:** Beginner
- **Objective:** Manage a todo list from the terminal.

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Add/list/complete tasks
2. Store tasks in a file
3. Id-based completion

## Architecture hints

Struct Task + serde optional or plain line format.

## Acceptance criteria

- Restarting the app keeps tasks

## Bonus challenges

- Due dates
- Tags

## Suggested layout

```text
p03-todo-cli/
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
