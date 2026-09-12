use std::cell::RefCell;
use std::rc::Rc;

struct Counter {
    inner: Rc<RefCell<i32>>,
}

impl Counter {
    fn new(start: i32) -> Self {
        let _ = start;
        unimplemented!("Implement Counter::new")
    }

    fn bump(&self, by: i32) {
        let _ = (self, by);
        unimplemented!("Implement bump")
    }

    fn get(&self) -> i32 {
        let _ = self;
        unimplemented!("Implement get")
    }

    fn share(&self) -> Self {
        let _ = self;
        unimplemented!("Implement share")
    }
}

fn main() {
    let a = Counter::new(0);
    let b = a.share();
    a.bump(2);
    b.bump(3);
    println!("a={} b={}", a.get(), b.get());
}
