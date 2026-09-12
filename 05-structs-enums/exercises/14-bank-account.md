# Exercise 14 — Bank Account

- **Title:** Bank Account
- **Objective:** Mutate struct state through methods with validation.
- **Difficulty:** Intermediate
- **Concepts practiced:** `&mut self`, `Result`

## Instructions

1. Define `struct BankAccount { owner: String, balance: i64 }` (cents).
2. Implement `deposit(&mut self, amount: i64) -> Result<(), String>`.
3. Implement `withdraw(&mut self, amount: i64) -> Result<(), String>`.
4. Reject non-positive amounts and overdrafts.

## Expected behavior

- Deposit 500 then withdraw 200 leaves balance 300.
- Withdraw 1000 from 300 returns `Err`.

## Optional hints

- Use `checked` arithmetic or simple comparisons.

## Bonus challenge

Track a transaction history `Vec<String>`.
