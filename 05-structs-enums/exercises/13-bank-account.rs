#[derive(Debug, PartialEq)]
enum TxError {
    ZeroAmount,
    InsufficientFunds { needed: u64, available: u64 },
}

#[derive(Debug)]
struct Account {
    owner: String,
    balance: u64, // cents
}

impl Account {
    fn new(owner: &str) -> Account {
        // TODO: implement
        let _ = owner;
        todo!("Create an account with balance 0")
    }

    fn deposit(&mut self, amount: u64) -> Result<u64, TxError> {
        // TODO: implement
        let _ = amount;
        todo!("Reject 0 with TxError::ZeroAmount, otherwise add and return the new balance")
    }

    fn withdraw(&mut self, amount: u64) -> Result<u64, TxError> {
        // TODO: implement
        let _ = amount;
        todo!("Reject 0 and overdrafts (InsufficientFunds) before mutating the balance")
    }

    fn transfer(&mut self, to: &mut Account, amount: u64) -> Result<(), TxError> {
        // TODO: implement
        let _ = (to, amount);
        todo!("Reuse withdraw and deposit with the ? operator")
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
