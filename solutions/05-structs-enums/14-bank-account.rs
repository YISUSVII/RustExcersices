struct BankAccount {
    owner: String,
    balance: i64,
}

impl BankAccount {
    fn new(owner: String) -> Self {
        Self { owner, balance: 0 }
    }

    fn deposit(&mut self, amount: i64) -> Result<(), String> {
        if amount <= 0 {
            return Err(String::from("deposit must be positive"));
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: i64) -> Result<(), String> {
        if amount <= 0 {
            return Err(String::from("withdraw must be positive"));
        }
        if amount > self.balance {
            return Err(String::from("insufficient funds"));
        }
        self.balance -= amount;
        Ok(())
    }
}

fn main() {
    let mut acct = BankAccount::new(String::from("Grace"));
    acct.deposit(500).unwrap();
    acct.withdraw(200).unwrap();
    println!("{} balance={}", acct.owner, acct.balance);
    println!("{:?}", acct.withdraw(1000));
}
