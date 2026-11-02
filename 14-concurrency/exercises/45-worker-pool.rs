#![allow(dead_code)]
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // TODO: implement — spawn a thread that loops: lock, recv, release lock, run job;
        // break when recv() returns Err (channel closed).
        let _ = (id, receiver);
        todo!("Spawn the worker thread and store its JoinHandle")
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        // TODO: implement — assert!(size > 0, "thread pool size must be > 0")
        let _ = size;
        todo!("Create the channel, wrap the receiver in Arc<Mutex<_>>, spawn workers")
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO: implement
        let _ = f;
        todo!("Box the closure and send it through self.sender")
    }

    fn size(&self) -> usize {
        self.workers.len()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // TODO: implement — drop(self.sender.take()), then join every worker handle.
    }
}

fn main() {
    let results = Arc::new(Mutex::new(Vec::new()));
    {
        let pool = ThreadPool::new(3);
        println!("pool with {} workers", pool.size());
        for n in 1..=6u64 {
            let results = Arc::clone(&results);
            pool.execute(move || {
                let square = n * n;
                results.lock().unwrap().push((n, square));
            });
        }
    }
    let mut results = results.lock().unwrap().clone();
    results.sort();
    println!("results = {results:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn runs_all_jobs_before_shutdown() {
        let counter = Arc::new(AtomicUsize::new(0));
        {
            let pool = ThreadPool::new(4);
            for _ in 0..100 {
                let c = Arc::clone(&counter);
                pool.execute(move || {
                    c.fetch_add(1, Ordering::SeqCst);
                });
            }
        }
        assert_eq!(counter.load(Ordering::SeqCst), 100);
    }

    #[test]
    fn results_can_be_returned_through_a_channel() {
        let pool = ThreadPool::new(2);
        let (tx, rx) = mpsc::channel();
        for i in 0..10 {
            let tx = tx.clone();
            pool.execute(move || tx.send(i * 2).unwrap());
        }
        drop(tx);
        let mut got: Vec<i32> = rx.iter().collect();
        got.sort();
        assert_eq!(got, (0..10).map(|i| i * 2).collect::<Vec<_>>());
    }

    #[test]
    #[should_panic(expected = "size must be > 0")]
    fn zero_sized_pool_panics() {
        ThreadPool::new(0);
    }
}
