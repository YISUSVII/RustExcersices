use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Debug)]
struct Account {
    owner: String,
    balance: i64,
}

type SharedAccount = Rc<RefCell<Account>>;

fn new_account(owner: &str, balance: i64) -> SharedAccount {
    Rc::new(RefCell::new(Account {
        owner: owner.to_string(),
        balance,
    }))
}

/// A household member holding a handle to a shared (joint) account.
struct Member {
    name: String,
    account: SharedAccount,
}

impl Member {
    fn deposit(&self, amount: i64) {
        self.account.borrow_mut().balance += amount;
    }

    fn withdraw(&self, amount: i64) -> Result<(), String> {
        let mut acc = self.account.borrow_mut();
        if acc.balance < amount {
            return Err(format!(
                "{} cannot withdraw {amount}: balance is {}",
                self.name, acc.balance
            ));
        }
        acc.balance -= amount;
        Ok(())
    }
}

fn transfer(from: &SharedAccount, to: &SharedAccount, amount: i64) -> Result<(), String> {
    if Rc::ptr_eq(from, to) {
        return Err("cannot transfer to the same account".to_string());
    }
    let mut src = from.borrow_mut();
    if src.balance < amount {
        return Err(format!("insufficient funds in {}'s account", src.owner));
    }
    src.balance -= amount;
    to.borrow_mut().balance += amount;
    Ok(())
}

/// A custom smart pointer that logs when it is dropped.
struct Tracked<T: std::fmt::Debug> {
    label: &'static str,
    value: T,
    drop_log: Rc<RefCell<Vec<String>>>,
}

impl<T: std::fmt::Debug> Tracked<T> {
    fn new(label: &'static str, value: T, drop_log: Rc<RefCell<Vec<String>>>) -> Self {
        Tracked {
            label,
            value,
            drop_log,
        }
    }
}

impl<T: std::fmt::Debug> Deref for Tracked<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T: std::fmt::Debug> DerefMut for Tracked<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T: std::fmt::Debug> Drop for Tracked<T> {
    fn drop(&mut self) {
        let msg = format!("dropping {} = {:?}", self.label, self.value);
        println!("{msg}");
        self.drop_log.borrow_mut().push(msg);
    }
}

fn main() {
    let joint = new_account("Ana & Ben", 100);
    let ana = Member {
        name: "Ana".into(),
        account: Rc::clone(&joint),
    };
    let ben = Member {
        name: "Ben".into(),
        account: Rc::clone(&joint),
    };
    println!("strong_count(joint) = {}", Rc::strong_count(&joint));

    ana.deposit(50);
    ben.withdraw(30).unwrap();
    println!("joint after ops: {:?}", joint.borrow());
    if let Err(e) = ben.withdraw(1_000) {
        println!("error: {e}");
    }

    let savings = new_account("Savings", 0);
    transfer(&joint, &savings, 70).unwrap();
    println!("joint = {}, savings = {}", joint.borrow().balance, savings.borrow().balance);

    let log = Rc::new(RefCell::new(Vec::new()));
    {
        let mut v = Tracked::new("numbers", vec![1, 2], Rc::clone(&log));
        v.push(3); // DerefMut lets us call Vec methods directly
        println!("len via Deref = {}", v.len());
    }
    println!("drop log: {:?}", log.borrow());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn members_share_the_same_account() {
        let acc = new_account("x", 10);
        let a = Member { name: "a".into(), account: Rc::clone(&acc) };
        let b = Member { name: "b".into(), account: Rc::clone(&acc) };
        a.deposit(5);
        b.withdraw(12).unwrap();
        assert_eq!(acc.borrow().balance, 3);
        assert_eq!(Rc::strong_count(&acc), 3);
        assert!(a.withdraw(100).is_err());
    }

    #[test]
    fn transfer_moves_money_and_rejects_overdraft() {
        let a = new_account("a", 50);
        let b = new_account("b", 0);
        transfer(&a, &b, 20).unwrap();
        assert_eq!((a.borrow().balance, b.borrow().balance), (30, 20));
        assert!(transfer(&a, &b, 100).is_err());
        assert!(transfer(&a, &a, 1).is_err());
    }

    #[test]
    fn tracked_derefs_and_logs_on_drop() {
        let log = Rc::new(RefCell::new(Vec::new()));
        {
            let mut t = Tracked::new("s", String::from("hi"), Rc::clone(&log));
            t.push('!');
            assert_eq!(t.len(), 3);
            assert!(log.borrow().is_empty());
        }
        assert_eq!(log.borrow().as_slice(), ["dropping s = \"hi!\""]);
    }
}
