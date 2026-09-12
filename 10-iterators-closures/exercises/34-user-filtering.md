# Exercise 34 — User Filtering

- **Title:** User Filtering
- **Objective:** Filter structs with closures.
- **Difficulty:** Advanced
- **Concepts practiced:** closures capturing environment

## Instructions

Given `struct User { name: String, age: u8 }`, implement
`fn names_at_least(users: &[User], min_age: u8) -> Vec<String>`.

## Expected behavior

- Users aged >= 18 return only their names in original order.

## Optional hints

- `filter(|u| u.age >= min_age).map(|u| u.name.clone())`.

## Bonus challenge

Return `&str` names with an explicit lifetime instead of cloning.
