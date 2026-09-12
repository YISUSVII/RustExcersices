# Project — Expense Tracker

- **Slug:** `p08-expense-tracker`
- **Difficulty:** Intermediate
- **Objective:** Record expenses and summarize by category.

## Requirements

- Implement as a Cargo binary (and library when it helps).
- Prefer standard library first; add crates only when educationally useful.
- Handle errors with `Result` rather than unchecked `unwrap` in non-demo paths.

## Suggested milestones

1. Add expense
2. List
3. Sum by category/month

## Architecture hints

CSV or JSON storage; struct Expense.

## Acceptance criteria

- Totals equal sum of entries

## Bonus challenges

- Budgets and alerts

## Suggested layout

```text
p08-expense-tracker/
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
