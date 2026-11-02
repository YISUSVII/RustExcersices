# Exercise 37 — Multi-Module Project

- **Title:** Multi-Module Project
- **Objective:** Organize a mini bank library into a module tree with deliberate visibility, then map that tree onto a real Cargo project layout.
- **Difficulty:** Advanced
- **Concepts practiced:** `mod`, `pub`, `pub(crate)`, `pub(super)`, private modules, `use`, `self::`, `super::`, `crate::` paths, re-exports (`pub use`), library vs binary crates, workspaces

## Instructions

The starter is a single file that simulates a library with inline `mod` blocks. The module tree and visibility markers are given; implement the `todo!()` bodies **and** be ready to explain why each item has the visibility it has.

```text
crate
├── bank                      (mod bank)
│   ├── errors                (pub mod)      BankError + Display
│   ├── money                 (pub(crate))   format_cents, validate_amount
│   ├── account               (pub mod)      Account (fields partly private)
│   ├── ledger                (private mod)  Entry, Ledger
│   ├── service               (pub mod)      Bank
│   └── pub use … re-exports  Account, BankError, Bank
└── main / run                "binary" side, uses only bank::{Bank, BankError}
```

1. `bank::money`:
   - `pub(crate) fn format_cents(cents: u64) -> String` — `12345` → `"$123.45"`.
   - `pub(crate) fn validate_amount(cents: u64) -> Result<u64, BankError>` — `0` is `InvalidAmount`.
2. `impl Display for BankError` — one message per variant; use `super::money::format_cents` for amounts.
3. `bank::account::Account`:
   - `pub(super) fn deposit(&mut self, cents: u64) -> Result<(), BankError>`
   - `pub(super) fn withdraw(&mut self, cents: u64) -> Result<(), BankError>` — `InsufficientFunds { needed, available }` on overdraft.
4. `bank::service::Bank`:
   - `pub fn open_account(&mut self, owner: &str) -> u32` (ids start at 1)
   - `pub fn account(&self, id: u32) -> Result<&Account, BankError>`
   - `pub fn deposit / withdraw(&mut self, id: u32, cents: u64) -> Result<(), BankError>`
   - `pub fn transfer(&mut self, from: u32, to: u32, cents: u64) -> Result<(), BankError>` — `SameAccount` if `from == to`; a failed transfer must leave all balances and the ledger unchanged.
   - `pub fn total_deposits_cents(&self) -> u64`
   - `pub fn statement(&self) -> Vec<String>` — one line per ledger entry (use the absolute path `crate::bank::money::format_cents`).
   - Record every successful operation in the private `ledger`.
5. Answer in comments: why can `main` call `bank::money::format_cents` but would **not** be able to if `bank` were an external crate? Why is `ledger` private yet `Bank` can use it?

## Expected behavior

```text
Expected failure: insufficient funds: needed $500.00, available $30.00
#1 Alice  $57.50
#2 Bob    $30.00
Total held: $87.50
Statement (5 entries):
  opened #1
  opened #2
  #1 +$100.00
  #1 -$12.50
  #1 -> #2 $30.00
```

- `bank.deposit(id, 0)` => `Err(BankError::InvalidAmount)`
- `bank.transfer(a, 77, 500)` => `Err(BankError::AccountNotFound(77))` and `a`'s balance is unchanged.
- Calling `Account::new(...)` or `account.deposit(...)` from `main` does **not** compile (`pub(super)`).

## Mapping to a real Cargo project

The same tree split into files (Rust 2018+ style, no `mod.rs` required):

```text
mini_bank/
├── Cargo.toml
└── src/
    ├── lib.rs              // pub mod bank;  (or put bank's contents directly here)
    ├── main.rs             // use mini_bank::bank::{Bank, BankError};  fn main() { … }
    ├── bank.rs             // pub mod errors; pub(crate) mod money; pub mod account;
    │                       // mod ledger; pub mod service;
    │                       // pub use self::{account::Account, errors::BankError, service::Bank};
    └── bank/
        ├── errors.rs
        ├── money.rs
        ├── account.rs
        ├── ledger.rs
        └── service.rs
```

- `mod foo;` (with a semicolon) tells the compiler to load `foo.rs` or `foo/mod.rs` next to the parent file.
- `src/lib.rs` is the **library crate** root; `src/main.rs` is a separate **binary crate** that depends on the library by package name (`mini_bank::…`). Inside `main.rs`, `crate::` refers to the binary, not the library.
- `pub(crate)` items in the library (like `money`) are **not** visible from `main.rs`, because the binary is a different crate. Expose what the binary needs via `pub` / `pub use` instead.
- Tests: unit tests stay in `#[cfg(test)] mod tests` inside each file; integration tests go in `tests/*.rs` and can only use the public API.

## Optional hints

- `super::` = parent module, `self::` = current module, `crate::` = crate root.
- `pub(super)` on `Account::new` lets `bank::service` build accounts while hiding the constructor from everyone else.
- In `transfer`, check `self.account(to)?` *before* withdrawing from `from`.
- `ok_or(BankError::AccountNotFound(id))` converts `Option` to `Result`.

## Bonus challenge

Turn it into a **Cargo workspace**:

```text
bank_workspace/
├── Cargo.toml          # [workspace] members = ["bank_core", "bank_cli", "shared_utils"]
├── shared_utils/       # lib: format_cents, validate_amount
├── bank_core/          # lib: depends on shared_utils = { path = "../shared_utils" }
└── bank_cli/           # bin: depends on bank_core = { path = "../bank_core" }
```

Run `cargo new --lib shared_utils`, etc., wire the path dependencies, and verify `cargo test --workspace` and `cargo run -p bank_cli` both work. Notice what had to change from `pub(crate)` to `pub` once `money` lived in another crate.
