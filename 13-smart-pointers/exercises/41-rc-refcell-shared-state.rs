#![allow(dead_code)]
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
    // TODO: implement
    let _ = (owner, balance);
    todo!("Wrap an Account in Rc::new(RefCell::new(...))")
}

struct Member {
    name: String,
    account: SharedAccount,
}

impl Member {
    fn deposit(&self, amount: i64) {
        // TODO: implement
        let _ = amount;
        todo!("Use self.account.borrow_mut()")
    }

    fn withdraw(&self, amount: i64) -> Result<(), String> {
        // TODO: implement
        let _ = amount;
        todo!("Return Err if balance < amount")
    }
}

fn transfer(from: &SharedAccount, to: &SharedAccount, amount: i64) -> Result<(), String> {
    // TODO: implement
    let _ = (from, to, amount);
    todo!("Reject Rc::ptr_eq(from, to) and overdrafts, then move the money")
}

struct Tracked<T: std::fmt::Debug> {
    label: &'static str,
    value: T,
    drop_log: Rc<RefCell<Vec<String>>>,
}

impl<T: std::fmt::Debug> Tracked<T> {
    fn new(label: &'static str, value: T, drop_log: Rc<RefCell<Vec<String>>>) -> Self {
        Tracked { label, value, drop_log }
    }
}

impl<T: std::fmt::Debug> Deref for Tracked<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // TODO: implement
        todo!("Return a reference to the inner value")
    }
}

impl<T: std::fmt::Debug> DerefMut for Tracked<T> {
    fn deref_mut(&mut self) -> &mut T {
        // TODO: implement
        todo!("Return a mutable reference to the inner value")
    }
}

impl<T: std::fmt::Debug> Drop for Tracked<T> {
    fn drop(&mut self) {
        // TODO: implement — push format!("dropping {} = {:?}", label, value) to drop_log
    }
}

fn main() {
    let joint = new_account("Ana & Ben", 100);
    let ana = Member { name: "Ana".into(), account: Rc::clone(&joint) };
    let ben = Member { name: "Ben".into(), account: Rc::clone(&joint) };
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
        v.push(3);
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
