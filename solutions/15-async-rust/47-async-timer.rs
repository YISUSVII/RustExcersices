use std::future::Future;
use std::pin::{pin, Pin};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};
use std::time::{Duration, Instant};

/// Wakes the executor by unparking the thread that is blocked in `block_on`.
struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.unpark();
    }
}

/// Minimal single-future executor: poll, and park the thread until woken.
fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut cx = Context::from_waker(&waker);
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(output) => return output,
            // `park` may wake spuriously; that's fine, we just poll again.
            Poll::Pending => thread::park(),
        }
    }
}

#[derive(Default)]
struct TimerState {
    completed: bool,
    waker: Option<Waker>,
}

/// A future that completes after `duration`, driven by a background thread.
struct TimerFuture {
    state: Arc<Mutex<TimerState>>,
}

impl TimerFuture {
    fn new(duration: Duration) -> TimerFuture {
        let state = Arc::new(Mutex::new(TimerState::default()));
        let thread_state = Arc::clone(&state);
        thread::spawn(move || {
            thread::sleep(duration);
            let mut s = thread_state.lock().unwrap();
            s.completed = true;
            if let Some(waker) = s.waker.take() {
                waker.wake();
            }
        });
        TimerFuture { state }
    }
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut s = self.state.lock().unwrap();
        if s.completed {
            Poll::Ready(())
        } else {
            // Always store the latest waker: the future may have moved to another task.
            match &s.waker {
                Some(w) if w.will_wake(cx.waker()) => {}
                _ => s.waker = Some(cx.waker().clone()),
            }
            Poll::Pending
        }
    }
}

fn sleep(duration: Duration) -> TimerFuture {
    TimerFuture::new(duration)
}

async fn countdown(from: u32, tick: Duration) -> u32 {
    let mut ticks = 0;
    for n in (1..=from).rev() {
        println!("  {n}...");
        sleep(tick).await;
        ticks += 1;
    }
    ticks
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
