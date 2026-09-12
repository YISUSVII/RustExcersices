# Level 4 — Ownership and Borrowing

## What you will learn

- Stack vs heap
- Ownership, moves, copies, cloning
- References and borrowing
- Mutable borrowing and borrowing rules
- Slices and string slices

## Simple explanation

Ownership is Rust's core memory-safety model. Every value has one owner, and when ownership moves, the previous binding is no longer valid. Borrowing lets you reference data without taking ownership.

## Small syntax examples

```rust
fn len(s: &String) -> usize {
    s.len()
}

fn main() {
    let text = String::from("rust");
    println!("{}", len(&text));
}
```

```rust
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}
```

## Common mistakes

- Using a moved value after ownership transfer
- Creating multiple mutable borrows simultaneously
- Returning references to local values

## Exercises

- Fix move vs clone compiler error
- Update a `String` via mutable borrowing
- Return string slices safely

See `exercises/`.

## Hints

- Read compiler errors fully; they usually explain the exact rule violated.
- Prefer borrowing (`&T` / `&mut T`) before cloning.

## Bonus challenges

- Refactor ownership-heavy code to reduce clones.

## Important compiler errors to understand

- `E0382`: use of moved value
- `E0502`: cannot borrow as mutable because it is also borrowed as immutable
- `E0515`: cannot return reference to local variable

## References

- <https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html>
- <https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html>
- <https://doc.rust-lang.org/book/ch04-03-slices.html>
