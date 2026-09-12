mod account {
    pub struct Account {
        owner: String,
        balance: i64,
    }

    impl Account {
        pub fn new(owner: String, balance: i64) -> Self {
            Self { owner, balance }
        }

        pub fn owner(&self) -> &str {
            &self.owner
        }

        pub fn balance(&self) -> i64 {
            self.balance
        }
    }

    pub mod audit {
        use super::Account;

        pub fn summary(acct: &Account) -> String {
            format!("{}: {}", acct.owner(), acct.balance())
        }
    }
}

fn main() {
    let acct = account::Account::new(String::from("Ada"), 100);
    println!("{}", account::audit::summary(&acct));
}
