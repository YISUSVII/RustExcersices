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
    let futures: Vec<_> = futures.into_iter().map(|f| Some(Box::pin(f))).collect();
    let outputs = futures.iter().map(|_| None).collect();
    JoinAll { futures, outputs }
}

// Every field is boxed or plain data, so JoinAll is Unpin regardless of F.
impl<F: Future> Unpin for JoinAll<F> {}

impl<F: Future> Future for JoinAll<F> {
    type Output = Vec<F::Output>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        let mut all_done = true;
        for (slot, out) in this.futures.iter_mut().zip(this.outputs.iter_mut()) {
            if let Some(fut) = slot {
                match fut.as_mut().poll(cx) {
                    Poll::Ready(v) => {
                        *out = Some(v);
                        *slot = None; // never poll a completed future again
                    }
                    Poll::Pending => all_done = false,
                }
            }
        }
        if all_done {
            Poll::Ready(this.outputs.iter_mut().map(|o| o.take().unwrap()).collect())
        } else {
            Poll::Pending
        }
    }
}

// ---------- fake requests ----------

#[derive(Debug, PartialEq)]
struct Response {
    id: usize,
    body: String,
}

async fn fake_request(id: usize, latency: Duration) -> Response {
    sleep(latency).await;
    Response {
        id,
        body: format!("payload #{id} after {}ms", latency.as_millis()),
    }
}

async fn fetch_sequential(latencies: &[u64]) -> Vec<Response> {
    let mut out = Vec::new();
    for (id, &ms) in latencies.iter().enumerate() {
        out.push(fake_request(id, Duration::from_millis(ms)).await);
    }
    out
}

async fn fetch_concurrent(latencies: &[u64]) -> Vec<Response> {
    join_all(
        latencies
            .iter()
            .enumerate()
            .map(|(id, &ms)| fake_request(id, Duration::from_millis(ms))),
    )
    .await
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
