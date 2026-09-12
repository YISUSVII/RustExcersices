# Exercise 19 — Match Guards

- **Title:** Match Guards
- **Objective:** Classify numbers with patterns and guards.
- **Difficulty:** Intermediate
- **Concepts practiced:** match guards, ranges

## Instructions

Implement `fn classify(n: i32) -> &'static str`:
- `0` => `"zero"`
- negative even => `"negative even"`
- negative odd => `"negative odd"`
- positive even => `"positive even"`
- positive odd => `"positive odd"`

## Expected behavior

- `classify(-4)` => `"negative even"`
- `classify(7)` => `"positive odd"`

## Optional hints

- Use arms like `n if n < 0 && n % 2 == 0`.

## Bonus challenge

Also classify multiples of 10 specially.
