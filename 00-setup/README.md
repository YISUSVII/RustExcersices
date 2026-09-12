# Level 0 — Setup

## What you will learn

- Installing Rust with `rustup`
- Using `rustc` and `cargo`
- Creating and running your first project
- Running quality tools: `cargo fmt` and `cargo clippy`

## Simple explanation

Rust projects are usually built with Cargo, Rust's official package manager and build tool. `rustc` compiles a single file directly, while Cargo helps manage complete projects.

## Small syntax examples

```bash
cargo new hello_rust
cd hello_rust
cargo run
cargo build
cargo check
cargo fmt
cargo clippy
```

```rust
fn main() {
    println!("Hello, Rust!");
}
```

## Common mistakes

- Not restarting the terminal after `rustup` installation
- Running `cargo run` outside a Cargo project folder
- Ignoring `cargo fmt` / `cargo clippy` warnings early

## Exercises

| # | Exercise | Path |
| - | -------- | ---- |
| 00a | Install and verify | `exercises/00a-install-verify.md` |
| 00b | First project checklist | `exercises/00b-first-project.md` |
| 00c | Hello starter (optional in-repo) | `exercises/00c-hello-rust/` |

## Hints

- `cargo new <project_name>` creates a complete starter project.
- `cargo check` is faster than `cargo build` while iterating.

## Bonus challenge

Print your name and learning goal in the Hello World app.

## References

- <https://www.rust-lang.org/tools/install>
- <https://doc.rust-lang.org/book/ch01-00-getting-started.html>
- <https://doc.rust-lang.org/cargo/>
