use std::sync::mpsc;
use std::thread;

fn square_pipeline(inputs: Vec<i32>) -> Vec<i32> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for n in inputs {
            tx.send(n).unwrap();
        }
    });
    rx.into_iter().map(|n| n * n).collect()
}

fn main() {
    println!("{:?}", square_pipeline(vec![1, 2, 3]));
}
