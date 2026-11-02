use std::collections::HashMap;
use std::future::Future;
use std::pin::{pin, Pin};
use std::sync::atomic::{AtomicUsize, Ordering};
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

struct Sleep {
    state: Arc<Mutex<TimerState>>,
}

fn sleep(duration: Duration) -> Sleep {
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
    Sleep { state }
}

impl Future for Sleep {
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

// ---------- timeout: a two-branch "select" ----------

#[derive(Debug, PartialEq)]
struct Elapsed;

struct Timeout<F> {
    inner: Pin<Box<F>>,
    deadline: Sleep,
}

fn timeout<F: Future>(duration: Duration, fut: F) -> Timeout<F> {
    Timeout {
        inner: Box::pin(fut),
        deadline: sleep(duration),
    }
}

impl<F: Future> Future for Timeout<F> {
    type Output = Result<F::Output, Elapsed>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Poll the real work first so a just-finished result wins over the deadline.
        if let Poll::Ready(v) = self.inner.as_mut().poll(cx) {
            return Poll::Ready(Ok(v));
        }
        match Pin::new(&mut self.deadline).poll(cx) {
            Poll::Ready(()) => Poll::Ready(Err(Elapsed)),
            Poll::Pending => Poll::Pending,
        }
    }
}

// ---------- fake HTTP ----------

#[derive(Debug, Clone, PartialEq)]
struct Request {
    method: &'static str,
    path: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Response {
    status: u16,
    body: String,
}

#[derive(Debug, Clone, PartialEq)]
enum TransportError {
    ConnectionReset,
}

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// An "async trait": methods return boxed futures, which is what the
/// `async-trait` crate generates and what keeps the trait object-safe.
trait Transport {
    fn send(&self, req: Request) -> BoxFuture<'_, Result<Response, TransportError>>;
}

struct FakeTransport {
    routes: HashMap<String, (u16, String)>,
    latency: Duration,
    failures_left: Mutex<usize>,
    calls: AtomicUsize,
}

impl FakeTransport {
    fn new(latency: Duration) -> Self {
        FakeTransport {
            routes: HashMap::new(),
            latency,
            failures_left: Mutex::new(0),
            calls: AtomicUsize::new(0),
        }
    }

    fn route(mut self, path: &str, status: u16, body: &str) -> Self {
        self.routes.insert(path.to_string(), (status, body.to_string()));
        self
    }

    /// The first `n` requests fail with `ConnectionReset`.
    fn failing_first(self, n: usize) -> Self {
        *self.failures_left.lock().unwrap() = n;
        self
    }

    fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}

impl Transport for FakeTransport {
    fn send(&self, req: Request) -> BoxFuture<'_, Result<Response, TransportError>> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            sleep(self.latency).await;
            let should_fail = {
                let mut left = self.failures_left.lock().unwrap();
                let fail = *left > 0;
                *left = left.saturating_sub(1);
                fail
            };
            if should_fail {
                return Err(TransportError::ConnectionReset);
            }
            let (status, body) = self
                .routes
                .get(&req.path)
                .cloned()
                .unwrap_or((404, format!("{} {} not found", req.method, req.path)));
            Ok(Response { status, body })
        })
    }
}

// ---------- client ----------

#[derive(Debug, PartialEq)]
enum ApiError {
    Timeout,
    Transport(TransportError),
    Status(u16),
}

impl ApiError {
    fn is_retryable(&self) -> bool {
        match self {
            ApiError::Timeout | ApiError::Transport(_) => true,
            ApiError::Status(code) => *code >= 500,
        }
    }
}

struct ApiClient<T: Transport> {
    transport: T,
    max_retries: u32,
    timeout: Duration,
    backoff: Duration,
}

impl<T: Transport> ApiClient<T> {
    fn new(transport: T) -> Self {
        ApiClient {
            transport,
            max_retries: 2,
            timeout: Duration::from_millis(100),
            backoff: Duration::from_millis(5),
        }
    }

    async fn get(&self, path: &str) -> Result<String, ApiError> {
        let mut attempt = 0;
        loop {
            let req = Request {
                method: "GET",
                path: path.to_string(),
            };
            let result = match timeout(self.timeout, self.transport.send(req)).await {
                Err(Elapsed) => Err(ApiError::Timeout),
                Ok(Err(e)) => Err(ApiError::Transport(e)),
                Ok(Ok(resp)) if (200..300).contains(&resp.status) => Ok(resp.body),
                Ok(Ok(resp)) => Err(ApiError::Status(resp.status)),
            };
            match result {
                Err(e) if e.is_retryable() && attempt < self.max_retries => {
                    attempt += 1;
                    println!("  attempt {attempt} failed with {e:?}, retrying...");
                    sleep(self.backoff * attempt).await;
                }
                other => return other,
            }
        }
    }
}

fn main() {
    let transport = FakeTransport::new(Duration::from_millis(20))
        .route("/users/1", 200, r#"{"id":1,"name":"Ferris"}"#)
        .route("/broken", 503, "unavailable")
        .failing_first(2);
    let client = ApiClient::new(transport);

    block_on(async {
        println!("GET /users/1 -> {:?}", client.get("/users/1").await);
        println!("calls so far: {}", client.transport.calls());
        println!("GET /missing -> {:?}", client.get("/missing").await);
        println!("GET /broken  -> {:?}", client.get("/broken").await);
    });

    let slow = ApiClient {
        timeout: Duration::from_millis(30),
        max_retries: 1,
        ..ApiClient::new(FakeTransport::new(Duration::from_millis(100)))
    };
    let start = Instant::now();
    let res = block_on(slow.get("/anything"));
    println!("slow server -> {res:?} after {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client(t: FakeTransport) -> ApiClient<FakeTransport> {
        ApiClient::new(t)
    }

    #[test]
    fn successful_request() {
        let c = client(FakeTransport::new(Duration::from_millis(5)).route("/ok", 200, "hi"));
        assert_eq!(block_on(c.get("/ok")), Ok("hi".to_string()));
        assert_eq!(c.transport.calls(), 1);
    }

    #[test]
    fn retries_transient_failures() {
        let t = FakeTransport::new(Duration::from_millis(5))
            .route("/ok", 200, "hi")
            .failing_first(2);
        let c = client(t);
        assert_eq!(block_on(c.get("/ok")), Ok("hi".to_string()));
        assert_eq!(c.transport.calls(), 3);
    }

    #[test]
    fn client_errors_are_not_retried() {
        let c = client(FakeTransport::new(Duration::from_millis(5)));
        assert_eq!(block_on(c.get("/nope")), Err(ApiError::Status(404)));
        assert_eq!(c.transport.calls(), 1);
    }

    #[test]
    fn times_out_and_gives_up_after_retries() {
        let c = ApiClient {
            timeout: Duration::from_millis(20),
            max_retries: 1,
            ..client(FakeTransport::new(Duration::from_millis(100)))
        };
        assert_eq!(block_on(c.get("/slow")), Err(ApiError::Timeout));
        assert_eq!(c.transport.calls(), 2);
    }

    #[test]
    fn timeout_passes_fast_futures_through() {
        assert_eq!(block_on(timeout(Duration::from_millis(50), async { 7 })), Ok(7));
    }
}
