# Exercise 21 — Grade Tracker

- **Title:** Grade Tracker
- **Objective:** Store student scores and compute averages.
- **Difficulty:** Intermediate
- **Concepts practiced:** `HashMap<String, Vec<u32>>`

## Instructions

1. Implement `fn add_score(book: &mut HashMap<String, Vec<u32>>, name: &str, score: u32)`.
2. Implement `fn average(book: &HashMap<String, Vec<u32>>, name: &str) -> Option<f64>`.

## Expected behavior

- After scores 80 and 100 for `"Ada"`, average is `90.0`.
- Missing student returns `None`.

## Optional hints

- Use `entry(...).or_default()`.

## Bonus challenge

Return the class top student.
