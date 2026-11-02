# Exercise 45 — Worker Pool

- **Title:** Worker Pool
- **Objective:** Build a fixed-size thread pool that executes boxed closures from a shared job queue and shuts down gracefully when dropped.
- **Difficulty:** Advanced
- **Concepts practiced:** `Box<dyn FnOnce() + Send + 'static>`, `mpsc` channels, `Arc<Mutex<Receiver<_>>>`, `Drop`, `Option::take`, `JoinHandle::join`

## Instructions

1. Define `type Job = Box<dyn FnOnce() + Send + 'static>;`.
2. Define `struct Worker { id: usize, handle: Option<thread::JoinHandle<()>> }` with `fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker`. The worker loops: lock the receiver, `recv()` a job, release the lock, run the job; exit the loop when `recv()` returns `Err`.
3. Define `struct ThreadPool { workers: Vec<Worker>, sender: Option<mpsc::Sender<Job>> }` and implement:
   - `fn new(size: usize) -> ThreadPool` — panics with `"thread pool size must be > 0"` if `size == 0`
   - `fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static`
   - `fn size(&self) -> usize`
4. Implement `Drop for ThreadPool`: drop the sender (closing the channel), then `join` every worker.

## Expected behavior

- Submitting 100 jobs to a 4-worker pool and then dropping the pool runs all 100 jobs.
- Dropping the pool prints `worker N shut down` for each worker.
- `ThreadPool::new(0)` panics.

## Optional hints

- Write `let job = receiver.lock().unwrap().recv();` on its own line so the `MutexGuard` is dropped **before** running the job; `while let Ok(job) = receiver.lock().unwrap().recv()` keeps the lock for the whole loop body.
- `Option::take()` lets you move the `Sender`/`JoinHandle` out of `&mut self` inside `drop`.

## Bonus challenge

Make `execute` return a `mpsc::Receiver<T>` so callers can get a job's result: `fn submit<T, F>(&self, f: F) -> mpsc::Receiver<T> where F: FnOnce() -> T + Send + 'static, T: Send + 'static`.
