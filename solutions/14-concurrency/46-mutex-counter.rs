use std::sync::{Arc, Mutex};
use std::thread;

fn bump_many(threads: usize, times: usize) -> usize {
    let counter = Arc::new(Mutex::new(0usize));
    let mut handles = Vec::new();
    for _ in 0..threads {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..times {
                *counter.lock().unwrap() += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let value = *counter.lock().unwrap();
    value
}

fn main() {
    println!("{}", bump_many(8, 1000));
}
