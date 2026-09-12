mod account {
    pub struct Account {
        owner: String,
        balance: i64,
    }

    impl Account {
        pub fn new(owner: String, balance: i64) -> Self {
            Self { owner, balance }
        }

        // TODO: add whatever accessors audit/main need without making fields public
    }

    pub mod audit {
        use super::Account;

        pub fn summary(acct: &Account) -> String {
            // TODO: format owner + balance using allowed API
            let _ = acct;
            unimplemented!("Implement audit::summary")
        }
    }
}

fn main() {
    let acct = account::Account::new(String::from("Ada"), 100);
    println!("{}", account::audit::summary(&acct));
}
