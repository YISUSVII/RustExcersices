# Exercise 13 — Bank Account

- **Title:** Bank Account
- **Objective:** Protect an account's invariants by validating operations with `Result`.
- **Difficulty:** Intermediate
- **Concepts practiced:** structs, enums as error types, `Result`, `&mut self`, `match`

## Instructions

Implement `Account`:

1. `Account::new(owner: &str) -> Account` with a balance of `0`.
2. `deposit(&mut self, amount: u64) -> Result<u64, TxError>` — rejects `0` with `TxError::ZeroAmount`; returns the new balance.
3. `withdraw(&mut self, amount: u64) -> Result<u64, TxError>` — rejects `0` and amounts larger than the balance (`TxError::InsufficientFunds { needed, available }`).
4. `transfer(&mut self, to: &mut Account, amount: u64) -> Result<(), TxError>`.

Balances are stored in cents (`u64`) to avoid floating-point rounding.

## Expected behavior

```text
deposit -> Ok(10000)
withdraw -> Err(InsufficientFunds { needed: 20000, available: 10000 })
transfer -> Ok(())
alice: 7500, bob: 2500
```

## Optional hints

- Validate first, mutate after: never leave an account half-updated.
- `transfer` can reuse `withdraw` and `deposit` with `?`.

## Bonus challenge

Keep a `Vec<Transaction>` history on each account.
