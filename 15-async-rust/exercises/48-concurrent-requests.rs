use std::future::Future;
use std::pin::{pin, Pin};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};
use std::time::{Duration, Instant};

// ---------- tiny runtime (see exercise 47) ----------

struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut cx = Context::from_waker(&waker);
    loop {
        if let Poll::Ready(out) = fut.as_mut().poll(&mut cx) {
            return out;
        }
        thread::park();
    }
}

#[derive(Default)]
struct TimerState {
    completed: bool,
    waker: Option<Waker>,
}

struct TimerFuture {
    state: Arc<Mutex<TimerState>>,
}

fn sleep(duration: Duration) -> TimerFuture {
    let state = Arc::new(Mutex::new(TimerState::default()));
    let thread_state = Arc::clone(&state);
    thread::spawn(move || {
        thread::sleep(duration);
        let mut s = thread_state.lock().unwrap();
        s.completed = true;
        if let Some(w) = s.waker.take() {
            w.wake();
        }
    });
    TimerFuture { state }
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut s = self.state.lock().unwrap();
        if s.completed {
            Poll::Ready(())
        } else {
            s.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

// ---------- join_all ----------

/// Polls every child future each time it is woken; resolves once all are done,
/// returning outputs in the original order.
struct JoinAll<F: Future> {
    futures: Vec<Option<Pin<Box<F>>>>,
    outputs: Vec<Option<F::Output>>,
}

fn join_all<I>(futures: I) -> JoinAll<I::Item>
where
    I: IntoIterator,
    I::Item: Future,
{
    // TODO: implement — Box::pin each future, and create one `None` output slot per future
    let _ = futures;
    todo!("Build the JoinAll struct")
}

impl<F: Future> Unpin for JoinAll<F> {}

impl<F: Future> Future for JoinAll<F> {
    type Output = Vec<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // TODO: implement
        // - poll every future that is still Some
        // - on Ready, store the output and set the slot to None
        // - when every slot is None, return Ready(outputs in order)
        let _ = cx;
        todo!("Poll all pending children")
    }
}

// ---------- fake requests ----------

#[derive(Debug, PartialEq)]
struct Response {
    id: usize,
    body: String,
}

async fn fake_request(id: usize, latency: Duration) -> Response {
    // TODO: implement — await sleep(latency), then return a Response
    let _ = (id, latency);
    todo!("Simulate a network call")
}

async fn fetch_sequential(latencies: &[u64]) -> Vec<Response> {
    // TODO: implement — await each fake_request one after another
    let _ = latencies;
    todo!("Sequential version")
}

async fn fetch_concurrent(latencies: &[u64]) -> Vec<Response> {
    // TODO: implement — build all request futures, then join_all(...).await
    let _ = latencies;
    todo!("Concurrent version")
}

fn main() {
    let latencies = [80, 30, 100, 50, 60];
    let sum: u64 = latencies.iter().sum();
    let max = latencies.iter().max().unwrap();

    let start = Instant::now();
    let seq = block_on(fetch_sequential(&latencies));
    println!("sequential: {} responses in {:?} (sum = {sum}ms)", seq.len(), start.elapsed());

    let start = Instant::now();
    let conc = block_on(fetch_concurrent(&latencies));
    println!("concurrent: {} responses in {:?} (max = {max}ms)", conc.len(), start.elapsed());
    for r in &conc {
        println!("  [{}] {}", r.id, r.body);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_all_preserves_order() {
        let responses = block_on(fetch_concurrent(&[40, 10, 20]));
        let ids: Vec<usize> = responses.iter().map(|r| r.id).collect();
        assert_eq!(ids, vec![0, 1, 2]);
    }

    #[test]
    fn join_all_of_ready_futures() {
        let out = block_on(join_all((1..=4).map(|n| async move { n * 10 })));
        assert_eq!(out, vec![10, 20, 30, 40]);
        let empty: Vec<i32> = block_on(join_all(Vec::<std::future::Ready<i32>>::new()));
        assert!(empty.is_empty());
    }

    #[test]
    fn concurrent_is_faster_than_sequential_sum() {
        // Sum is 400ms; concurrent should be close to 80ms. Bound is generous.
        let start = Instant::now();
        let r = block_on(fetch_concurrent(&[80, 80, 80, 80, 80]));
        let elapsed = start.elapsed();
        assert_eq!(r.len(), 5);
        assert!(elapsed >= Duration::from_millis(80));
        assert!(elapsed < Duration::from_millis(300), "took {elapsed:?}");
    }
}
