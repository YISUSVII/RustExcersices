#[derive(Debug, PartialEq)]
enum TxError {
    ZeroAmount,
    InsufficientFunds { needed: u64, available: u64 },
}

#[derive(Debug, PartialEq)]
enum Transaction {
    Deposit(u64),
    Withdrawal(u64),
}

#[derive(Debug)]
struct Account {
    owner: String,
    balance: u64, // cents
    history: Vec<Transaction>,
}

impl Account {
    fn new(owner: &str) -> Account {
        Account {
            owner: owner.to_string(),
            balance: 0,
            history: Vec::new(),
        }
    }

    fn deposit(&mut self, amount: u64) -> Result<u64, TxError> {
        if amount == 0 {
            return Err(TxError::ZeroAmount);
        }
        self.balance += amount;
        self.history.push(Transaction::Deposit(amount));
        Ok(self.balance)
    }

    fn withdraw(&mut self, amount: u64) -> Result<u64, TxError> {
        if amount == 0 {
            return Err(TxError::ZeroAmount);
        }
        if amount > self.balance {
            return Err(TxError::InsufficientFunds {
                needed: amount,
                available: self.balance,
            });
        }
        self.balance -= amount;
        self.history.push(Transaction::Withdrawal(amount));
        Ok(self.balance)
    }

    fn transfer(&mut self, to: &mut Account, amount: u64) -> Result<(), TxError> {
        self.withdraw(amount)?;
        to.deposit(amount)?;
        Ok(())
    }
}

fn main() {
    let mut alice = Account::new("alice");
    let mut bob = Account::new("bob");

    println!("deposit -> {:?}", alice.deposit(10_000));
    println!("withdraw -> {:?}", alice.withdraw(20_000));
    println!("transfer -> {:?}", alice.transfer(&mut bob, 2_500));
    println!(
        "{}: {}, {}: {}",
        alice.owner, alice.balance, bob.owner, bob.balance
    );
    println!("{} history: {:?}", alice.owner, alice.history);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero() {
        let mut a = Account::new("a");
        assert_eq!(a.deposit(0), Err(TxError::ZeroAmount));
        assert_eq!(a.withdraw(0), Err(TxError::ZeroAmount));
    }

    #[test]
    fn rejects_overdraft_without_mutation() {
        let mut a = Account::new("a");
        a.deposit(100).unwrap();
        assert_eq!(
            a.withdraw(150),
            Err(TxError::InsufficientFunds {
                needed: 150,
                available: 100
            })
        );
        assert_eq!(a.balance, 100);
    }

    #[test]
    fn transfer_moves_money() {
        let mut a = Account::new("a");
        let mut b = Account::new("b");
        a.deposit(100).unwrap();
        a.transfer(&mut b, 40).unwrap();
        assert_eq!((a.balance, b.balance), (60, 40));
        assert!(a.transfer(&mut b, 1_000).is_err());
        assert_eq!((a.balance, b.balance), (60, 40));
    }
}
