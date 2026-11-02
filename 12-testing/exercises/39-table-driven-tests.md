# Exercise 39 — Table-Driven Tests

- **Title:** Table-Driven Tests
- **Objective:** Test a small expression calculator and a password checker with data tables, cover every error variant, and use the tests to find and fix real bugs.
- **Difficulty:** Advanced
- **Concepts practiced:** table-driven tests (arrays/slices of tuples), helper assertions with `#[track_caller]`, descriptive failure messages, testing `Result` errors and `Display` output, regression-driven bug fixing

## Instructions

The starter provides:

- `enum Op { Add, Sub, Mul, Div }` and `enum CalcError { Empty, BadNumber(String), UnknownOperator(String), WrongTokenCount(usize), DivisionByZero, Overflow }` with a `Display` impl.
- `evaluate(expr: &str) -> Result<i64, CalcError>` for expressions like `"12 * -3"` (operators `+ - * x /`).
- `password_issues(pw: &str) -> Vec<&'static str>` returning failed rules among `"too short"`, `"no digit"`, `"no uppercase"`, `"no lowercase"`.

The code has **two hidden bugs**. Do not fix them yet!

1. Implement the helper:
   ```rust
   #[track_caller]
   fn check_eval(name: &str, input: &str, expected: Result<i64, CalcError>)
   ```
   It must fail with a message such as `case 'div by zero': evaluate("1 / 0") returned ..., expected ...`.
2. `evaluates_valid_expressions` — a table of `(name, input, expected)` rows: addition, subtraction to a negative, `*` and `x`, negative operands, truncating division, extra whitespace.
3. `reports_each_error_kind` — a table with at least one row for **every** `CalcError` variant, including `"9223372036854775807 + 1"` (overflow) and `"1 / 0"`.
4. `error_messages_are_human_readable` — a table of `(CalcError, &str)` checking `to_string()`.
5. `password_rules_table` — a `&[(&str, &[&str])]` table from "everything wrong" to "strong".
6. Run the tests. They will **panic** inside `evaluate` instead of returning errors. Use the failure output to locate the two bugs, fix them with `checked_*` arithmetic, and re-run until everything is green.

## Expected behavior

- Before fixing: the error-case test panics with `attempt to add with overflow` / `attempt to divide by zero`.
- After fixing: `test result: ok. 4 passed; 0 failed`.
- `evaluate("4 % 2")` => `Err(UnknownOperator("%"))`
- `evaluate("1 + 2 + 3")` => `Err(WrongTokenCount(5))`
- `evaluate("-9223372036854775808 / -1")` => `Err(Overflow)`
- `password_issues("Str0ngPassword")` => `[]`

## Optional hints

- Arrays of tuples iterate by value: `for (name, input, expected) in cases { ... }`.
- For slices of references, destructure with `for &(pw, expected) in cases`.
- `#[track_caller]` makes the panic location point at the table loop's call site rather than the helper.
- Give every row a **name** — when a table has 10 rows, "case 7 failed" is not helpful.
- `i64::checked_add`, `checked_div`, ... return `Option<i64>`; `.ok_or(CalcError::Overflow)` converts it.

## Bonus challenge

- Write a tiny `macro_rules! cases` that generates one `#[test]` function per row so each case shows up separately in the test output.
- Add a property-style test: for a range of `a` and non-zero `b`, check `evaluate(&format!("{a} / {b}"))` equals `Ok(a / b)`.
