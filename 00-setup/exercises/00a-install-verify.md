# Exercise 00a — Install and Verify

- **Title:** Install and Verify Toolchain
- **Objective:** Confirm Rust tooling works on your machine.
- **Difficulty:** Beginner
- **Concepts practiced:** rustup, PATH, versions

## Instructions

1. Install Rust via `rustup` if needed.
2. Run `rustc --version` and `cargo --version`.
3. Run `rustup show` and note the active toolchain.
4. Check off the boxes in this file locally as you go.

## Checklist

- [ ] `rustc --version` prints a stable version
- [ ] `cargo --version` works in the same shell
- [ ] `rustup component list` shows `rustfmt` / `clippy` available (install if missing)

## Expected behavior

Commands exit 0 and print version strings.

## Bonus challenge

Install nightly toolchain and switch back to stable.
