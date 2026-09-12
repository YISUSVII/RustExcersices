# Project — Word Counter

- **Slug:** `p05-word-counter`
- **Difficulty:** Beginner
- **Objective:** Count lines, words, and characters in a file (wc-like).

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Read file path arg
2. Compute stats
3. Print summary

## Architecture hints

std::fs + iterators.

## Acceptance criteria

- Matches counts on a fixture file you include

## Bonus challenges

- Multiple files
- Freq table top-N

## Suggested layout

```text
p05-word-counter/
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
