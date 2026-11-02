# Exercise 27 — Custom Errors

- **Title:** Custom Error Type for a Config Loader
- **Objective:** Design an application error enum that wraps lower-level errors so `?` works across different error types.
- **Difficulty:** Advanced
- **Concepts practiced:** error enums, `impl Display`, `impl std::error::Error` with `source()`, `From` conversions, `?`, `Box<dyn Error>`

## Instructions

1. Define `AppError` with variants:
   - `Io(std::io::Error)`
   - `Parse { key: String, source: std::num::ParseIntError }`
   - `MissingKey(&'static str)`
   - `Invalid(String)`
2. Implement `std::fmt::Display` (short, human message) and `std::error::Error` — override `source()` to return the wrapped error for `Io` and `Parse`.
3. Implement `From<std::io::Error> for AppError` so `fs::read_to_string(path)?` converts automatically.
4. Implement `parse_config(text: &str) -> Result<Config, AppError>` for `key=value` lines (`#` comments and blank lines ignored). Required keys: `host` (non-empty), `port` (`u16`, not `0`), `workers` (`u8`, `1..=64`).
5. Implement `load_config(path: &Path) -> Result<Config, AppError>` using `?` on the file read and on `parse_config`.
6. In `main`, write sample config files under `std::env::temp_dir()`, load a valid one, then show each error kind, printing the `source()` chain. Clean up the files afterwards.

## Expected behavior

```text
loaded: Config { host: "localhost", port: 8080, workers: 4 }
error: I/O error
  caused by: No such file or directory (os error 2)
error: invalid value for 'port'
  caused by: invalid digit found in string
error: missing key 'workers'
error: invalid config: workers must be in 1..=64, got 99
```

(The exact I/O message depends on your OS.)

## Optional hints

- `"8080".parse::<u16>().map_err(|e| AppError::Parse { key: "port".into(), source: e })?`
- Walk the chain with `let mut src = err.source(); while let Some(e) = src { ...; src = e.source(); }`.
- Returning `Result<(), Box<dyn Error>>` from a function lets `?` accept *any* error type.

## Bonus challenge

Add a `line: usize` field to `Invalid` errors so users know where the problem is.
