// A mini bank "library" laid out with inline modules.
// Each `mod name { ... }` block corresponds to a file in a real Cargo project
// (see the exercise README for the mapping).

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
                match self {
                    BankError::AccountNotFound(id) => write!(f, "account #{id} not found"),
                    BankError::InsufficientFunds { needed, available } => write!(
                        f,
                        "insufficient funds: needed {}, available {}",
                        super::money::format_cents(*needed),
                        super::money::format_cents(*available)
                    ),
                    BankError::InvalidAmount => write!(f, "amount must be greater than zero"),
                    BankError::SameAccount => write!(f, "cannot transfer to the same account"),
                }
            }
        }

        impl std::error::Error for BankError {}
    }

    // Crate-visible helpers: usable anywhere inside this crate, hidden from library users.
    pub(crate) mod money {
        pub(crate) fn format_cents(cents: u64) -> String {
            format!("${}.{:02}", cents / 100, cents % 100)
        }

        pub(crate) fn validate_amount(cents: u64) -> Result<u64, super::errors::BankError> {
            if cents == 0 {
                Err(super::errors::BankError::InvalidAmount)
            } else {
                Ok(cents)
            }
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
            // Only the bank module may create accounts, so ids stay unique.
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
                self.balance_cents += money::validate_amount(cents)?;
                Ok(())
            }

            pub(super) fn withdraw(&mut self, cents: u64) -> Result<(), BankError> {
                money::validate_amount(cents)?;
                if cents > self.balance_cents {
                    return Err(BankError::InsufficientFunds {
                        needed: cents,
                        available: self.balance_cents,
                    });
                }
                self.balance_cents -= cents;
                Ok(())
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

    // Re-exports form the public API: callers write `bank::Bank`, not `bank::service::Bank`.
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
                let id = self.next_id;
                self.next_id += 1;
                self.accounts.insert(id, Account::new(id, owner));
                self.ledger.record(Entry::Opened { id });
                id
            }

            pub fn account(&self, id: u32) -> Result<&Account, BankError> {
                self.accounts.get(&id).ok_or(BankError::AccountNotFound(id))
            }

            fn account_mut(&mut self, id: u32) -> Result<&mut Account, BankError> {
                self.accounts
                    .get_mut(&id)
                    .ok_or(BankError::AccountNotFound(id))
            }

            pub fn deposit(&mut self, id: u32, cents: u64) -> Result<(), BankError> {
                self.account_mut(id)?.deposit(cents)?;
                self.ledger.record(Entry::Deposit { id, cents });
                Ok(())
            }

            pub fn withdraw(&mut self, id: u32, cents: u64) -> Result<(), BankError> {
                self.account_mut(id)?.withdraw(cents)?;
                self.ledger.record(Entry::Withdrawal { id, cents });
                Ok(())
            }

            pub fn transfer(&mut self, from: u32, to: u32, cents: u64) -> Result<(), BankError> {
                if from == to {
                    return Err(BankError::SameAccount);
                }
                // Validate the destination first so a failed transfer changes nothing.
                self.account(to)?;
                self.account_mut(from)?.withdraw(cents)?;
                self.account_mut(to)?.deposit(cents)?;
                self.ledger.record(Entry::Transfer { from, to, cents });
                Ok(())
            }

            pub fn total_deposits_cents(&self) -> u64 {
                self.accounts.values().map(Account::balance_cents).sum()
            }

            pub fn history_len(&self) -> usize {
                self.ledger.entries().len()
            }

            pub fn statement(&self) -> Vec<String> {
                self.ledger
                    .entries()
                    .iter()
                    .map(|e| match e {
                        Entry::Opened { id } => format!("opened #{id}"),
                        Entry::Deposit { id, cents } => {
                            format!("#{id} +{}", crate::bank::money::format_cents(*cents))
                        }
                        Entry::Withdrawal { id, cents } => {
                            format!("#{id} -{}", crate::bank::money::format_cents(*cents))
                        }
                        Entry::Transfer { from, to, cents } => format!(
                            "#{from} -> #{to} {}",
                            crate::bank::money::format_cents(*cents)
                        ),
                    })
                    .collect()
            }
        }
    }
}

// The "binary crate" side: it only uses the public API.
//
// Q: Why can `run` call `bank::money::format_cents`?
// A: `money` is `pub(crate)` and `run` lives in the same crate. If `bank` were a
//    separate library crate (src/lib.rs) and this were src/main.rs, it would be a
//    different crate and `pub(crate)` items would be invisible.
//
// Q: Why can `Bank` use the private `ledger` module?
// A: Private items are visible to the module that declares them and all of its
//    descendants. `ledger` is declared in `bank`, and `service` is a child of `bank`.
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
    if let Err(e) = bank.deposit(99, 100) {
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

    #[test]
    fn money_helper_is_crate_visible() {
        assert_eq!(crate::bank::money::format_cents(12_345), "$123.45");
        assert_eq!(crate::bank::money::format_cents(5), "$0.05");
    }
}
