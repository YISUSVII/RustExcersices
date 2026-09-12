struct BankAccount {
    owner: String,
    balance: i64,
}

impl BankAccount {
    fn new(owner: String) -> Self {
        Self { owner, balance: 0 }
    }

    fn deposit(&mut self, amount: i64) -> Result<(), String> {
        // TODO: reject amount <= 0, otherwise add to balance
        let _ = (self, amount);
        unimplemented!("Implement deposit")
    }

    fn withdraw(&mut self, amount: i64) -> Result<(), String> {
        // TODO: reject amount <= 0 or amount > balance
        let _ = (self, amount);
        unimplemented!("Implement withdraw")
    }
}

fn main() {
    let mut acct = BankAccount::new(String::from("Grace"));
    acct.deposit(500).unwrap();
    acct.withdraw(200).unwrap();
    println!("{} balance={}", acct.owner, acct.balance);
    println!("{:?}", acct.withdraw(1000));
}
