# Exercise 41 — Rc + RefCell Shared State

- **Title:** Rc + RefCell Shared State
- **Objective:** Model a joint bank account shared by several owners, and build a custom smart pointer with `Deref` and `Drop`.
- **Difficulty:** Intermediate
- **Concepts practiced:** `Rc<T>`, `RefCell<T>`, interior mutability, `Rc::strong_count`, `Rc::ptr_eq`, `Deref`/`DerefMut`, `Drop`

## Instructions

1. Define `struct Account { owner: String, balance: i64 }` and `type SharedAccount = Rc<RefCell<Account>>;`.
2. Implement `fn new_account(owner: &str, balance: i64) -> SharedAccount`.
3. Define `struct Member { name: String, account: SharedAccount }` and implement:
   - `fn deposit(&self, amount: i64)`
   - `fn withdraw(&self, amount: i64) -> Result<(), String>` (error if the balance is insufficient)
4. Implement `fn transfer(from: &SharedAccount, to: &SharedAccount, amount: i64) -> Result<(), String>`. Reject transfers to the same account (use `Rc::ptr_eq`) and overdrafts.
5. Build a custom smart pointer `Tracked<T: Debug> { label, value, drop_log: Rc<RefCell<Vec<String>>> }` with:
   - `fn new(label: &'static str, value: T, drop_log: Rc<RefCell<Vec<String>>>) -> Self`
   - `impl Deref` / `impl DerefMut` targeting `T`
   - `impl Drop` that pushes `"dropping {label} = {value:?}"` to the log.

## Expected behavior

- Two `Member`s sharing an account see each other's deposits; `Rc::strong_count` is 3 (original + 2 members).
- `transfer(&a, &a, 1)` → `Err(..)`.
- A `Tracked<Vec<i32>>` can call `push`/`len` directly thanks to `Deref`, and logs `dropping numbers = [1, 2, 3]` when it goes out of scope.

## Optional hints

- `RefCell::borrow_mut` panics if the value is already borrowed — keep borrows short and never hold two `borrow_mut()` on the same `RefCell` (that is why `ptr_eq` matters in `transfer`).
- `Drop::drop` runs automatically at the end of scope; you never call it directly (use `std::mem::drop(x)` to drop early).

## Bonus challenge

Use `RefCell::try_borrow_mut` in `transfer` to return an error instead of panicking if the account is already borrowed.
