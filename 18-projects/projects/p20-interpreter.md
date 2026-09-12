# Project — Small Language Interpreter

- **Slug:** `p20-interpreter`
- **Difficulty:** Ninja
- **Objective:** Interpret a tiny language (lets, arithmetic, if, functions optional).

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Lexer
2. Parser AST
3. Evaluator

## Architecture hints

Start with arithmetic + variables.

## Acceptance criteria

- Runs sample programs to expected values

## Bonus challenges

- Functions
- REPL

## Suggested layout

```text
p20-interpreter/
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
