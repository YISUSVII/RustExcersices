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

1. **Install and verify**: Install Rust and confirm `rustc --version` and `cargo --version` work.
2. **First project**: Create `hello_rust` and run it.
3. **Tooling drill**: Run `cargo check`, `cargo fmt`, and `cargo clippy`.

## Hints

- `cargo new <project_name>` creates a complete starter project.
- `cargo check` is faster than `cargo build` while iterating.

## Bonus challenge

Print your name and learning goal in the Hello World app.

## References

- <https://www.rust-lang.org/tools/install>
- <https://doc.rust-lang.org/book/ch01-00-getting-started.html>
- <https://doc.rust-lang.org/cargo/>
