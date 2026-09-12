# Project — Simple Shell

- **Slug:** `p17-simple-shell`
- **Difficulty:** Ninja
- **Objective:** Minimal shell executing external programs with pipes/redirection subset.

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Parse command line
2. spawn processes
3. implement cd/exit builtins

## Architecture hints

std::process::Command.

## Acceptance criteria

- Runs external commands and builtins

## Bonus challenges

- Pipes
- Background jobs

## Suggested layout

```text
p17-simple-shell/
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
