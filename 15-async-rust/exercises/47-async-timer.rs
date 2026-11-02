#![allow(dead_code)]
use std::future::Future;
#[allow(unused_imports)]
use std::pin::{pin, Pin};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};
use std::time::{Duration, Instant};

/// Wakes the executor by unparking the thread blocked in `block_on`.
struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        // TODO: implement — unpark the stored thread
    }
}

fn block_on<F: Future>(fut: F) -> F::Output {
    // TODO: implement
    // 1. pin the future with std::pin::pin!
    // 2. build a Waker from Arc<ThreadWaker(thread::current())>
    // 3. loop { poll; if Pending -> thread::park() }
    let _ = fut;
    todo!("Write a minimal executor")
}

#[derive(Default)]
struct TimerState {
    completed: bool,
    waker: Option<Waker>,
}

struct TimerFuture {
    state: Arc<Mutex<TimerState>>,
}

impl TimerFuture {
    fn new(duration: Duration) -> TimerFuture {
        // TODO: implement — spawn a thread that sleeps, sets completed = true and wakes the stored waker
        let _ = duration;
        todo!("Create shared state and the timer thread")
    }
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // TODO: implement — Ready if completed, otherwise store cx.waker().clone() and return Pending
        let _ = cx;
        todo!("Check the shared state")
    }
}

fn sleep(duration: Duration) -> TimerFuture {
    TimerFuture::new(duration)
}

async fn countdown(from: u32, tick: Duration) -> u32 {
    // TODO: implement — print each number, await sleep(tick), return the number of ticks
    let _ = (from, tick);
    todo!("Loop from `from` down to 1, awaiting a timer each time")
}

fn main() {
    let value = block_on(async { 6 * 7 });
    println!("block_on(async {{ 6 * 7 }}) = {value}");

    let start = Instant::now();
    block_on(sleep(Duration::from_millis(100)));
    println!("slept for ~{:?}", start.elapsed());

    println!("countdown:");
    let start = Instant::now();
    let ticks = block_on(countdown(3, Duration::from_millis(50)));
    println!("liftoff after {ticks} ticks in ~{:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_on_ready_future() {
        assert_eq!(block_on(async { "done" }), "done");
    }

    #[test]
    fn timer_waits_at_least_its_duration() {
        let start = Instant::now();
        block_on(sleep(Duration::from_millis(30)));
        assert!(start.elapsed() >= Duration::from_millis(30));
    }

    #[test]
    fn sequential_awaits_add_up() {
        let start = Instant::now();
        let ticks = block_on(countdown(3, Duration::from_millis(20)));
        assert_eq!(ticks, 3);
        assert!(start.elapsed() >= Duration::from_millis(60));
    }

    #[test]
    fn timer_is_pending_before_completion() {
        let mut timer = sleep(Duration::from_millis(50));
        let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
        let mut cx = Context::from_waker(&waker);
        assert!(Pin::new(&mut timer).poll(&mut cx).is_pending());
        block_on(timer);
    }
}
