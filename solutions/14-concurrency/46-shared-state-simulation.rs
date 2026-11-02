use std::sync::{Arc, Mutex, MutexGuard, PoisonError, RwLock};
use std::thread;

#[derive(Debug, PartialEq)]
enum TransferError {
    SameAccount,
    UnknownAccount(usize),
    InsufficientFunds { account: usize, balance: i64 },
}

struct Bank {
    accounts: Vec<Mutex<i64>>,
    /// Many readers (auditors), occasional writer (transfers).
    log: RwLock<Vec<String>>,
}

/// Locks a mutex, recovering the data even if another thread panicked while holding it.
fn lock_recover<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

impl Bank {
    fn new(balances: &[i64]) -> Bank {
        Bank {
            accounts: balances.iter().map(|&b| Mutex::new(b)).collect(),
            log: RwLock::new(Vec::new()),
        }
    }

    /// Moves `amount` from `from` to `to`. Always locks the lower index first so two
    /// opposite transfers (A→B and B→A) can never deadlock.
    fn transfer(&self, from: usize, to: usize, amount: i64) -> Result<(), TransferError> {
        if from == to {
            return Err(TransferError::SameAccount);
        }
        for idx in [from, to] {
            if idx >= self.accounts.len() {
                return Err(TransferError::UnknownAccount(idx));
            }
        }

        let (first, second) = if from < to { (from, to) } else { (to, from) };
        let mut g1 = lock_recover(&self.accounts[first]);
        let mut g2 = lock_recover(&self.accounts[second]);
        let (src, dst) = if from < to {
            (&mut *g1, &mut *g2)
        } else {
            (&mut *g2, &mut *g1)
        };

        if *src < amount {
            return Err(TransferError::InsufficientFunds {
                account: from,
                balance: *src,
            });
        }
        *src -= amount;
        *dst += amount;
        drop((g1, g2));

        self.log
            .write()
            .unwrap_or_else(PoisonError::into_inner)
            .push(format!("{from} -> {to}: {amount}"));
        Ok(())
    }

    fn balance(&self, idx: usize) -> i64 {
        *lock_recover(&self.accounts[idx])
    }

    /// Total money in the bank. Locks all accounts in index order for a consistent snapshot.
    fn total(&self) -> i64 {
        let guards: Vec<_> = self.accounts.iter().map(lock_recover).collect();
        guards.iter().map(|g| **g).sum()
    }

    fn log_len(&self) -> usize {
        self.log.read().unwrap_or_else(PoisonError::into_inner).len()
    }
}

/// Runs `rounds` transfers per thread across all account pairs in both directions.
fn simulate(bank: &Bank, threads: usize, rounds: usize) {
    let n = bank.accounts.len();
    thread::scope(|s| {
        for t in 0..threads {
            s.spawn(move || {
                for r in 0..rounds {
                    let from = (t + r) % n;
                    let to = (t + r * 3 + 1) % n;
                    let _ = bank.transfer(from, to, 1 + (r % 5) as i64);
                }
            });
        }
        // An auditor reading concurrently through the RwLock.
        s.spawn(|| {
            for _ in 0..10 {
                let _ = bank.log_len();
            }
        });
    });
}

fn main() {
    let bank = Bank::new(&[1_000, 1_000, 1_000, 1_000]);
    let before = bank.total();
    simulate(&bank, 8, 1_000);
    println!("total before = {before}, after = {}", bank.total());
    println!("transfers logged = {}", bank.log_len());
    for i in 0..4 {
        println!("account {i}: {}", bank.balance(i));
    }
    println!("{:?}", bank.transfer(0, 0, 1));
    println!("{:?}", bank.transfer(0, 1, 1_000_000));

    // Poisoning demo: a thread panics while holding the lock.
    let shared = Arc::new(Mutex::new(42));
    let s2 = Arc::clone(&shared);
    let _ = thread::spawn(move || {
        let _guard = s2.lock().unwrap();
        panic!("simulated crash while holding the lock");
    })
    .join();
    println!("poisoned = {}", shared.is_poisoned());
    println!("recovered value = {}", *lock_recover(&shared));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_is_conserved_under_concurrency() {
        let bank = Bank::new(&[500, 500, 500]);
        simulate(&bank, 6, 2_000);
        assert_eq!(bank.total(), 1_500);
        assert!((0..3).all(|i| bank.balance(i) >= 0));
    }

    #[test]
    fn transfer_errors() {
        let bank = Bank::new(&[10, 0]);
        assert_eq!(bank.transfer(0, 0, 1), Err(TransferError::SameAccount));
        assert_eq!(bank.transfer(0, 9, 1), Err(TransferError::UnknownAccount(9)));
        assert_eq!(
            bank.transfer(1, 0, 5),
            Err(TransferError::InsufficientFunds { account: 1, balance: 0 })
        );
        assert_eq!(bank.transfer(0, 1, 10), Ok(()));
        assert_eq!((bank.balance(0), bank.balance(1)), (0, 10));
        assert_eq!(bank.log_len(), 1);
    }

    #[test]
    fn recovers_from_poisoned_mutex() {
        let bank = Arc::new(Bank::new(&[100, 100]));
        let b = Arc::clone(&bank);
        let result = thread::spawn(move || {
            let _g = b.accounts[0].lock().unwrap();
            panic!("boom");
        })
        .join();
        assert!(result.is_err());
        assert!(bank.accounts[0].is_poisoned());
        assert_eq!(bank.transfer(0, 1, 50), Ok(()));
        assert_eq!(bank.total(), 200);
    }
}
