# Exercise 16 — Option/Result Branching

- **Title:** Option and Result Branching
- **Objective:** Combine `Option` and `Result` values and branch on every outcome explicitly.
- **Difficulty:** Intermediate
- **Concepts practiced:** `match` on `Result`/`Option`, nested patterns, `if let`, `ParseIntError`

## Instructions

1. Implement `parse_id(input: &str) -> Result<u32, std::num::ParseIntError>` (trim whitespace first).
2. Implement `lookup(id: u32) -> Option<&'static str>` using the provided `USERS` table.
3. Implement `greet(input: &str) -> String` with a single `match` on `parse_id(input).map(lookup)`:
   - valid id, user found → `"Hello, <name>!"`
   - valid id, no user → `"No user with id <id>"`
   - parse error → `"Invalid id '<input>': <error>"`
4. Implement `sum_valid(inputs: &[&str]) -> (u32, usize)` returning the sum of parseable ids and the number of invalid inputs. Use `if let` or `match`.

## Expected behavior

```text
Hello, ada!
No user with id 99
Invalid id 'abc': invalid digit found in string
sum: 6, invalid: 2
```

## Optional hints

- `Result<Option<&str>, _>` can be matched with nested patterns like `Ok(Some(name))`.
- You'll need the original id for the "not found" message — keep it around or match on `parse_id` first.

## Bonus challenge

Rewrite `sum_valid` with `filter_map` and `partition`.
