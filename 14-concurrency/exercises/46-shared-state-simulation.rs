#![allow(dead_code)]
#[allow(unused_imports)]
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
    log: RwLock<Vec<String>>,
}

fn lock_recover<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    // TODO: implement
    let _ = m;
    todo!("m.lock().unwrap_or_else(PoisonError::into_inner)")
}

impl Bank {
    fn new(balances: &[i64]) -> Bank {
        Bank {
            accounts: balances.iter().map(|&b| Mutex::new(b)).collect(),
            log: RwLock::new(Vec::new()),
        }
    }

    fn transfer(&self, from: usize, to: usize, amount: i64) -> Result<(), TransferError> {
        // TODO: implement — validate, lock lower index first, move money, log it
        let _ = (from, to, amount);
        todo!("Lock accounts in index order to avoid deadlock")
    }

    fn balance(&self, idx: usize) -> i64 {
        // TODO: implement
        let _ = idx;
        todo!("Read one account via lock_recover")
    }

    fn total(&self) -> i64 {
        // TODO: implement
        todo!("Lock every account in order, then sum")
    }

    fn log_len(&self) -> usize {
        // TODO: implement
        todo!("Take a read lock on self.log")
    }
}

fn simulate(bank: &Bank, threads: usize, rounds: usize) {
    // TODO: implement — use thread::scope to run transfers in both directions
    let _ = (bank, threads, rounds);
    todo!("Spawn scoped worker threads plus an auditor thread")
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
