# Exercise 54 — Safe Wrapper over Unsafe

- **Title:** Safe Wrapper
- **Objective:** Encapsulate unsafe pointer logic behind a safe API.
- **Difficulty:** Ninja
- **Concepts practiced:** `unsafe`, raw pointers, invariants

## Instructions

Implement `struct OwnedBuffer { ptr: *mut u8, len: usize }` with:
- `fn new(len: usize) -> Self` allocating zeroed memory via `std::alloc` **or** simpler: store a `Vec<u8>` privately and only demonstrate an unsafe read helper.
- Prefer the simpler educational version: keep `Vec<u8>` and implement `unsafe fn get_unchecked(&self, idx: usize) -> u8` used only inside a safe `fn get(&self, idx: usize) -> Option<u8>`.

## Expected behavior

- `get` returns `None` out of bounds and `Some` in bounds without UB.

## Optional hints

- Safety comment required above `unsafe` block.
- Never call `get_unchecked` with OOB indices.

## Bonus challenge

Implement `Drop` story if using manual allocation.
