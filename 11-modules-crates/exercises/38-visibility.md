# Exercise 38 — Visibility Drill

- **Title:** Visibility Drill
- **Objective:** Practice `pub` boundaries between parent and child modules.
- **Difficulty:** Advanced
- **Concepts practiced:** module tree, `pub(crate)`, privacy

## Instructions

In a single file module tree:
1. Parent module `account` has private field access only via methods.
2. Child module `account::audit` can read balance through a `pub(super)` or method API you design.
3. `main` must not read private fields directly.

Complete the TODOs so the file compiles and prints the audit line.

## Expected behavior

- Creating an account with 100 and auditing prints balance 100.

## Optional hints

- Private fields stay private; expose getters.

## Bonus challenge

Use `pub(crate)` instead of fully public getters.
