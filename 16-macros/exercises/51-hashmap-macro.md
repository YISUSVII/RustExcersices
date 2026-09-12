# Exercise 51 — hashmap! Macro

- **Title:** hashmap! Macro
- **Objective:** Build a `HashMap` from key => value pairs.
- **Difficulty:** Advanced
- **Concepts practiced:** macro matching, token trees

## Instructions

Implement `macro_rules! hashmap` supporting `hashmap!{"a" => 1, "b" => 2}`.

## Expected behavior

- Creates a map with the provided entries.

## Optional hints

- `$( $k:expr => $v:expr ),* $(,)?`

## Bonus challenge

Allow trailing commas and empty maps.
