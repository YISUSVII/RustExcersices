# Level 13 — Smart Pointers

## What you will learn
`Box<T>`, `Rc<T>`, `Arc<T>`, `RefCell<T>`, interior mutability, `Deref`, `Drop`.

## Simple explanation
Smart pointers add ownership behaviors like shared ownership or runtime borrow checking.

## Small syntax examples
```rust
use std::rc::Rc;
let shared = Rc::new(String::from("data"));
```

## Common mistakes
Reference cycles, unnecessary shared ownership, misusing `RefCell`.

## Exercises
Solve realistic ownership problems with appropriate pointer choices.

## Hints
Pick the smallest abstraction that fits your ownership needs.

## Bonus challenges
Prevent or detect `Rc` cycles in graph-like data.

## References
<https://doc.rust-lang.org/book/ch15-00-smart-pointers.html>
