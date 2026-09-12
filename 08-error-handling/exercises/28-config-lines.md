# Exercise 28 — Config Lines

- **Title:** Config Line Parser
- **Objective:** Parse `key=value` lines and collect a map, skipping comments/blanks.
- **Difficulty:** Intermediate
- **Concepts practiced:** line parsing, `Result`, `HashMap`

## Instructions

Implement `fn parse_config(text: &str) -> Result<HashMap<String, String>, String>`:
- ignore empty lines and lines starting with `#`
- each data line must contain `=`
- trim keys/values
- duplicate keys => error

## Expected behavior

```text
# comment
name = Ada
lang=rust
```
yields `name=Ada`, `lang=rust`.

## Optional hints

- Split on the first `=` only.

## Bonus challenge

Support quoted values.
