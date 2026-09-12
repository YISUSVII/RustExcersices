# Project — CLI Calculator

- **Slug:** `p01-cli-calculator`
- **Difficulty:** Beginner
- **Objective:** Build a command-line calculator supporting + - * / and parentheses-free two-operand expressions.

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Parse CLI args or interactive stdin
2. Implement operations with Result
3. Pretty-print results

## Architecture hints

Binary crate; keep parse/eval modules separate.

## Acceptance criteria

- `cargo run -- 3 + 4` prints 7
- Division by zero is a controlled error

## Bonus challenges

- Add % and power
- History of last 10 results

## Suggested layout

```text
p01-cli-calculator/
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
