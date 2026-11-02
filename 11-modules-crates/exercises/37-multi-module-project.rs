// A mini bank "library" laid out with inline modules.
// Each `mod name { ... }` block corresponds to a file in a real Cargo project.
// The module tree and visibility are given; fill in the TODOs.

mod bank {
    pub mod errors {
        use std::fmt;

        #[derive(Debug, Clone, PartialEq)]
        pub enum BankError {
            AccountNotFound(u32),
            InsufficientFunds { needed: u64, available: u64 },
            InvalidAmount,
            SameAccount,
        }

        impl fmt::Display for BankError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // TODO: write a message per variant.
                // Use `super::money::format_cents` to format amounts.
                let _ = f;
                todo!("match self and write! a friendly message")
            }
        }

        impl std::error::Error for BankError {}
    }

    // Crate-visible helpers: usable anywhere inside this crate, hidden from library users.
    pub(crate) mod money {
        pub(crate) fn format_cents(cents: u64) -> String {
            // TODO: implement
            let _ = cents;
            todo!("12345 -> \"$123.45\"")
        }

        pub(crate) fn validate_amount(cents: u64) -> Result<u64, super::errors::BankError> {
            // TODO: implement
            let _ = cents;
            todo!("Err(InvalidAmount) for 0, Ok(cents) otherwise")
        }
    }

    pub mod account {
        use super::errors::BankError;
        use super::money;

        #[derive(Debug, Clone)]
        pub struct Account {
            id: u32,
            pub owner: String,
            balance_cents: u64,
        }

        impl Account {
            // pub(super): only the `bank` module may create accounts.
            pub(super) fn new(id: u32, owner: &str) -> Self {
                Self {
                    id,
                    owner: owner.to_string(),
                    balance_cents: 0,
                }
            }

            pub fn id(&self) -> u32 {
                self.id
            }

            pub fn balance_cents(&self) -> u64 {
                self.balance_cents
            }

            pub fn balance_display(&self) -> String {
                money::format_cents(self.balance_cents)
            }

            pub(super) fn deposit(&mut self, cents: u64) -> Result<(), BankError> {
                // TODO: validate the amount, then add it
                let _ = cents;
                todo!("Use money::validate_amount and ?")
            }

            pub(super) fn withdraw(&mut self, cents: u64) -> Result<(), BankError> {
                // TODO: validate the amount and check the balance
                let _ = cents;
                todo!("Err(InsufficientFunds) when cents > balance")
            }
        }
    }

    // Private module: an implementation detail of `bank`.
    mod ledger {
        #[derive(Debug, Clone, PartialEq)]
        pub enum Entry {
            Opened { id: u32 },
            Deposit { id: u32, cents: u64 },
            Withdrawal { id: u32, cents: u64 },
            Transfer { from: u32, to: u32, cents: u64 },
        }

        #[derive(Debug, Default)]
        pub struct Ledger {
            entries: Vec<Entry>,
        }

        impl Ledger {
            pub fn record(&mut self, entry: Entry) {
                self.entries.push(entry);
            }

            pub fn entries(&self) -> &[Entry] {
                &self.entries
            }
        }
    }

    // Re-exports form the public API: callers write `bank::Bank`.
    pub use self::account::Account;
    pub use self::errors::BankError;
    pub use self::service::Bank;

    pub mod service {
        use super::ledger::{Entry, Ledger};
        use super::{Account, BankError};
        use std::collections::BTreeMap;

        #[derive(Debug, Default)]
        pub struct Bank {
            accounts: BTreeMap<u32, Account>,
            next_id: u32,
            ledger: Ledger,
        }

        impl Bank {
            pub fn new() -> Self {
                Self {
                    next_id: 1,
                    ..Self::default()
                }
            }

            pub fn open_account(&mut self, owner: &str) -> u32 {
                // TODO: assign next_id, insert Account::new, record Entry::Opened
                let _ = (owner, &mut self.accounts, &mut self.ledger);
                todo!("Return the new account id")
            }

            pub fn account(&self, id: u32) -> Result<&Account, BankError> {
                // TODO: implement
                let _ = id;
                todo!("Err(AccountNotFound) if missing")
            }

            pub fn deposit(&mut self, id: u32, cents: u64) -> Result<(), BankError> {
                // TODO: implement and record Entry::Deposit
                let _ = (id, cents);
                todo!()
            }

            pub fn withdraw(&mut self, id: u32, cents: u64) -> Result<(), BankError> {
                // TODO: implement and record Entry::Withdrawal
                let _ = (id, cents);
                todo!()
            }

            pub fn transfer(&mut self, from: u32, to: u32, cents: u64) -> Result<(), BankError> {
                // TODO: reject same account; make sure a failed transfer changes nothing
                let _ = (from, to, cents);
                todo!()
            }

            pub fn total_deposits_cents(&self) -> u64 {
                // TODO: implement
                todo!("Sum all balances")
            }

            pub fn history_len(&self) -> usize {
                self.ledger.entries().len()
            }

            pub fn statement(&self) -> Vec<String> {
                // TODO: turn each ledger entry into a line, e.g. "#1 +$100.00"
                // Hint: use the absolute path crate::bank::money::format_cents
                let _: &[Entry] = self.ledger.entries();
                todo!()
            }
        }
    }
}

// The "binary crate" side: it only uses the public API.
use bank::{Bank, BankError};

fn run() -> Result<(), BankError> {
    let mut bank = Bank::new();
    let alice = bank.open_account("Alice");
    let bob = bank.open_account("Bob");

    bank.deposit(alice, 10_000)?;
    bank.withdraw(alice, 1_250)?;
    bank.transfer(alice, bob, 3_000)?;

    if let Err(e) = bank.transfer(bob, alice, 50_000) {
        println!("Expected failure: {e}");
    }

    for id in [alice, bob] {
        let acct = bank.account(id)?;
        println!(
            "#{} {:<6} {}",
            acct.id(),
            acct.owner,
            acct.balance_display()
        );
    }
    println!(
        "Total held: {}",
        bank::money::format_cents(bank.total_deposits_cents())
    );

    println!("Statement ({} entries):", bank.history_len());
    for line in bank.statement() {
        println!("  {line}");
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::bank::{Bank, BankError};

    #[test]
    fn deposit_withdraw_and_balance() -> Result<(), BankError> {
        let mut bank = Bank::new();
        let id = bank.open_account("Ana");
        bank.deposit(id, 500)?;
        bank.withdraw(id, 200)?;
        assert_eq!(bank.account(id)?.balance_cents(), 300);
        assert_eq!(bank.account(id)?.balance_display(), "$3.00");
        Ok(())
    }

    #[test]
    fn errors_are_reported() {
        let mut bank = Bank::new();
        let id = bank.open_account("Ana");
        assert_eq!(bank.deposit(id, 0), Err(BankError::InvalidAmount));
        assert_eq!(
            bank.withdraw(id, 1),
            Err(BankError::InsufficientFunds {
                needed: 1,
                available: 0
            })
        );
        assert_eq!(bank.deposit(42, 1), Err(BankError::AccountNotFound(42)));
        assert_eq!(bank.transfer(id, id, 1), Err(BankError::SameAccount));
    }

    #[test]
    fn failed_transfer_changes_nothing() -> Result<(), BankError> {
        let mut bank = Bank::new();
        let a = bank.open_account("A");
        bank.deposit(a, 1_000)?;
        assert_eq!(
            bank.transfer(a, 77, 500),
            Err(BankError::AccountNotFound(77))
        );
        assert_eq!(bank.account(a)?.balance_cents(), 1_000);
        assert_eq!(bank.history_len(), 2);
        Ok(())
    }
}
