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
- [40 — Box Recursive List](exercises/40-box-recursive-list.md)
- [41 — Rc + RefCell Shared State](exercises/41-rc-refcell-shared-state.md)
- [42 — Weak Tree](exercises/42-weak-tree.md)

See `exercises/` for starters and `../solutions/13-smart-pointers/` for reference solutions.

## Hints
Pick the smallest abstraction that fits your ownership needs.

## Bonus challenges
Prevent or detect `Rc` cycles in graph-like data.

## References
<https://doc.rust-lang.org/book/ch15-00-smart-pointers.html>
