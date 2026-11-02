# Exercise 26 — Data Validation

- **Title:** Signup Form Validation
- **Objective:** Validate raw user input into a trusted type, reporting either the first error or every error.
- **Difficulty:** Intermediate
- **Concepts practiced:** "parse, don't validate", error enums with data, `impl Display`, `?`, collecting multiple errors

## Instructions

Raw input arrives as `SignupForm<'a> { username: &'a str, email: &'a str, age: &'a str, password: &'a str }`.

1. Implement one validator per field, each returning `Result<_, ValidationError>`:
   - `validate_username(&str) -> Result<String, ValidationError>` — trimmed, non-empty (`EmptyUsername`), at most 16 chars (`UsernameTooLong { max: 16 }`), only alphanumerics or `_` (`InvalidUsernameChar(char)`).
   - `validate_email(&str) -> Result<String, ValidationError>` — exactly one `@` with non-empty local part and a domain containing a `.` (`InvalidEmail(String)`).
   - `validate_age(&str) -> Result<u8, ValidationError>` — must parse (`AgeNotANumber(String)`) and be in `13..=120` (`AgeOutOfRange(u32)`).
   - `validate_password(&str) -> Result<(), ValidationError>` — at least 8 chars, one digit, one uppercase letter (`WeakPassword(&'static str)` with the reason).
2. Implement `std::fmt::Display` for `ValidationError` with human-readable messages.
3. Implement `validate(form: &SignupForm) -> Result<ValidUser, ValidationError>` — stops at the first error using `?`.
4. Implement `validate_all(form: &SignupForm) -> Result<ValidUser, Vec<ValidationError>>` — runs every validator and returns all errors at once.

## Expected behavior

With the sample data in `main`:

```text
valid: ValidUser { username: "ferris", email: "ferris@rust-lang.org", age: 30 }
first error: email 'nope' is invalid
all errors:
  - username contains invalid character '!'
  - email 'nope' is invalid
  - age must be a number, got 'ten'
  - password is too weak: needs at least 8 characters
```

## Optional hints

- Validate in `validate` with `let username = validate_username(form.username)?;` etc.
- In `validate_all`, `match` on each result and push errors into a `Vec`.
- Parse to `u32` first so `"500"` produces `AgeOutOfRange(500)` instead of a parse error.

## Bonus challenge

Implement `std::error::Error` for `ValidationError` and return `Box<dyn Error>` from a helper.
