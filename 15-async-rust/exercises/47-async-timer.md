# Exercise 47 — Async Timer

- **Title:** Async Timer
- **Objective:** Understand how futures, wakers, and executors fit together by writing a minimal `block_on` executor and a `TimerFuture` using only the standard library.
- **Difficulty:** Advanced
- **Concepts practiced:** `Future` trait, `Poll`, `Pin`, `Context`, `Waker`, `std::task::Wake`, thread parking, `async`/`.await`

> **Real projects use a runtime.** In production you'd write `#[tokio::main] async fn main()` and `tokio::time::sleep(..).await`. Tokio is an external crate, so this exercise builds the same pieces by hand with `std` only — that way you see what a runtime does for you, and the file compiles with plain `rustc`.

## Instructions

1. Implement a waker: `struct ThreadWaker(Thread);` with `impl Wake for ThreadWaker` whose `wake` calls `self.0.unpark()`.
2. Implement `fn block_on<F: Future>(fut: F) -> F::Output`:
   - pin the future (`std::pin::pin!`),
   - build `Waker::from(Arc::new(ThreadWaker(thread::current())))` and a `Context`,
   - loop: `poll`; on `Poll::Ready(v)` return `v`; on `Poll::Pending` call `thread::park()`.
3. Implement `struct TimerFuture { state: Arc<Mutex<TimerState>> }` where `TimerState { completed: bool, waker: Option<Waker> }`:
   - `fn TimerFuture::new(duration: Duration) -> TimerFuture` spawns a thread that sleeps, sets `completed = true`, and calls `wake()` on the stored waker.
   - `impl Future for TimerFuture` (`Output = ()`): return `Ready` if completed, otherwise store `cx.waker().clone()` and return `Pending`.
4. Add `fn sleep(duration: Duration) -> TimerFuture` and `async fn countdown(from: u32, tick: Duration) -> u32` that prints `from..=1` while awaiting a timer each tick, returning the number of ticks.

## Expected behavior

- `block_on(async { 6 * 7 })` → `42`.
- `block_on(sleep(Duration::from_millis(100)))` returns after ~100 ms.
- `block_on(countdown(3, 50ms))` prints `3... 2... 1...` and takes ~150 ms (awaits run **sequentially**).
- A freshly created timer returns `Poll::Pending` on its first poll.

## Optional hints

- A future does nothing until polled. `Pending` is a promise: "I stored your waker and will call it when I can make progress."
- The timer thread must lock the state, set `completed`, and take the waker **in the same critical section** that `poll` uses, or a wake-up could be lost.
- `thread::park` may return spuriously — that's why `block_on` polls in a loop.

## Bonus challenge

Compare with the Tokio version (requires `tokio = { version = "1", features = ["full"] }` in `Cargo.toml`):

```rust
use std::time::Duration;

#[tokio::main]
async fn main() {
    for n in (1..=3).rev() {
        println!("{n}...");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
```

Then extend your executor to run many tasks: keep a queue of `Pin<Box<dyn Future<Output = ()> + Send>>` tasks whose wakers re-enqueue them through an `mpsc::Sender`.
