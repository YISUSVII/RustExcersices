use std::cell::RefCell;
use std::rc::Rc;

struct Counter {
    inner: Rc<RefCell<i32>>,
}

impl Counter {
    fn new(start: i32) -> Self {
        Self {
            inner: Rc::new(RefCell::new(start)),
        }
    }

    fn bump(&self, by: i32) {
        *self.inner.borrow_mut() += by;
    }

    fn get(&self) -> i32 {
        *self.inner.borrow()
    }

    fn share(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

fn main() {
    let a = Counter::new(0);
    let b = a.share();
    a.bump(2);
    b.bump(3);
    println!("a={} b={}", a.get(), b.get());
}
