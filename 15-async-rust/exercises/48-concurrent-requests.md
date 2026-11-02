# Exercise 48 — Concurrent Requests

- **Title:** Concurrent Requests
- **Objective:** Run many simulated requests concurrently on a single thread with a hand-written `join_all`, and observe that total time ≈ the slowest request, not the sum.
- **Difficulty:** Advanced
- **Concepts practiced:** composing futures, `Pin<Box<F>>`, `Unpin`, polling multiple children, concurrency vs. parallelism, `async fn`

> **Real projects use a runtime.** With Tokio you'd write `tokio::join!(a, b)` or `futures::future::join_all(v).await`. This exercise is std-only so you can see how `join_all` works under the hood. The tiny `block_on`/`sleep` runtime from exercise 47 is provided.

## Instructions

1. Implement `struct JoinAll<F: Future> { futures: Vec<Option<Pin<Box<F>>>>, outputs: Vec<Option<F::Output>> }` and the constructor:
   ```rust
   fn join_all<I>(futures: I) -> JoinAll<I::Item> where I: IntoIterator, I::Item: Future
   ```
2. Implement `Future for JoinAll<F>` with `Output = Vec<F::Output>`: poll every still-pending child; store finished outputs and set their slot to `None`; return `Ready` with outputs **in the original order** once every child is done.
3. Implement `async fn fake_request(id: usize, latency: Duration) -> Response` (awaits `sleep(latency)`, returns `Response { id, body }`).
4. Implement:
   - `async fn fetch_sequential(latencies: &[u64]) -> Vec<Response>` — `.await` one after another.
   - `async fn fetch_concurrent(latencies: &[u64]) -> Vec<Response>` — build all futures, then `join_all(..).await`.
5. In `main`, time both versions with `Instant`.

## Expected behavior

For latencies `[80, 30, 100, 50, 60]` ms:

```text
sequential: 5 responses in ~320ms (sum = 320ms)
concurrent: 5 responses in ~100ms (max = 100ms)
```

- `join_all` returns outputs in input order, even if later futures finish first.
- `join_all` of an empty `Vec` resolves immediately to `vec![]`.

## Optional hints

- Calling an `async fn` only *creates* a future; nothing runs until it's polled. Sequential `.await`s therefore add up.
- Never poll a future again after it returned `Ready` — that's why completed slots become `None`.
- Boxing each child (`Box::pin`) makes `JoinAll` `Unpin`, so `poll` can use `&mut *self` freely.

## Bonus challenge

The Tokio equivalent (with `tokio` and `futures` crates):

```rust
#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(fake_request(0, ms(80)), fake_request(1, ms(30)));
    let all = futures::future::join_all((0..5).map(|i| fake_request(i, ms(50)))).await;
}
```

Write a `join2(a, b)` future returning `(A::Output, B::Output)`, and a `select2(a, b)` that returns whichever finishes first.
