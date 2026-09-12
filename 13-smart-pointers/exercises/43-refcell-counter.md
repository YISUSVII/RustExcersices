# Exercise 43 — RefCell Counter

- **Title:** RefCell Counter
- **Objective:** Mutate through shared ownership using interior mutability.
- **Difficulty:** Advanced
- **Concepts practiced:** `Rc<RefCell<T>>`

## Instructions

Implement a `struct Counter` holding `Rc<RefCell<i32>>` with:
- `fn new(start: i32) -> Self`
- `fn bump(&self, by: i32)`
- `fn get(&self) -> i32`
- `fn share(&self) -> Self` (shares the same cell)

## Expected behavior

- Two shared counters bumping both affect the same value.

## Optional hints

- `*self.inner.borrow_mut() += by`.

## Bonus challenge

Return `Result` from `try_bump` using `try_borrow_mut`.
