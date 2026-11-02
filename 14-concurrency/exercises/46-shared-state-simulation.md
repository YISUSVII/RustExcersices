# Exercise 46 — Shared State Simulation

- **Title:** Shared State Simulation
- **Objective:** Simulate concurrent bank transfers without data races or deadlocks, and recover from a poisoned mutex.
- **Difficulty:** Ninja
- **Concepts practiced:** `Mutex<T>`, `RwLock<T>`, lock ordering, deadlock avoidance, `PoisonError::into_inner`, `thread::scope`, `Arc<T>`

## Instructions

1. Define:
   ```rust
   enum TransferError { SameAccount, UnknownAccount(usize), InsufficientFunds { account: usize, balance: i64 } }
   struct Bank { accounts: Vec<Mutex<i64>>, log: RwLock<Vec<String>> }
   ```
2. Implement `fn lock_recover<T>(m: &Mutex<T>) -> MutexGuard<'_, T>` that returns the guard even if the mutex is poisoned.
3. Implement on `Bank`:
   - `fn new(balances: &[i64]) -> Bank`
   - `fn transfer(&self, from: usize, to: usize, amount: i64) -> Result<(), TransferError>` — **always lock the lower index first**, then append `"{from} -> {to}: {amount}"` to `log` (write lock).
   - `fn balance(&self, idx: usize) -> i64`
   - `fn total(&self) -> i64` — lock all accounts (in order) for a consistent snapshot.
   - `fn log_len(&self) -> usize` (read lock).
4. Implement `fn simulate(bank: &Bank, threads: usize, rounds: usize)` that uses `thread::scope` to run many transfers in both directions across accounts, plus an "auditor" thread reading the log.
5. In `main`, demonstrate poisoning: a thread panics while holding a lock; show `is_poisoned()` and recover the value with `lock_recover`.

## Expected behavior

- After `simulate(&bank, 8, 1_000)` on 4 accounts of 1000, `bank.total()` is still `4000` and the program never hangs.
- `transfer(0, 0, 1)` → `Err(SameAccount)`; overdrafts → `Err(InsufficientFunds { .. })`.
- A poisoned account mutex is still usable through `lock_recover`.

## Optional hints

- Deadlock: thread 1 locks A then waits for B while thread 2 locks B then waits for A. A global lock order (by index) makes that impossible.
- `m.lock().unwrap_or_else(PoisonError::into_inner)` recovers the guard from a poisoned lock.
- `RwLock` allows many simultaneous readers or one writer — good for an audit log read far more often than written.

## Bonus challenge

Replace per-account `Mutex<i64>` with `AtomicI64` and implement `transfer` with `compare_exchange` loops. Which invariants become harder to guarantee (hint: `total()` snapshots)?
