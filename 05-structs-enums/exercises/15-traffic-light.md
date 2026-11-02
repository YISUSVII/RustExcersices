# Exercise 15 — Traffic Light

- **Title:** Traffic Light Enum
- **Objective:** Model a traffic light with an enum and attach behavior to each variant.
- **Difficulty:** Beginner
- **Concepts practiced:** unit-like enums, `match`, `Copy` types, implementing `std::fmt::Display`

## Instructions

1. Use the enum `TrafficLight { Red, Yellow, Green }`.
2. Implement:
   - `next(self) -> TrafficLight` — `Red → Green → Yellow → Red`
   - `duration_secs(&self) -> u32` — Red `30`, Yellow `5`, Green `25`
   - `can_go(&self) -> bool` — only `Green`
3. Implement `std::fmt::Display` so lights print as `RED`, `YELLOW`, `GREEN`.
4. In `main`, simulate one full cycle and print the total cycle time.

## Expected behavior

```text
RED for 30s (go: false)
GREEN for 25s (go: true)
YELLOW for 5s (go: false)
cycle: 60s
```

## Optional hints

- Derive `Clone, Copy` so `next(self)` doesn't move the light away from you.
- `match` must cover all variants — the compiler checks this for you.

## Bonus challenge

Add a `FlashingYellow` variant and see which `match` expressions the compiler forces you to update.
