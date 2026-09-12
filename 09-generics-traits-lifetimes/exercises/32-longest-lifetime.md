# Exercise 32 — Longest with Lifetimes

- **Title:** Longest Lifetime
- **Objective:** Annotate lifetimes when returning a reference.
- **Difficulty:** Advanced
- **Concepts practiced:** lifetime parameters

## Instructions

Implement `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` returning the longer slice (tie => first).

## Expected behavior

- `longest("ab", "xyz")` => `"xyz"`

## Optional hints

- Both inputs and the output share `'a`.

## Bonus challenge

Write `longest_all<'a>(items: &[&'a str]) -> Option<&'a str>`.
