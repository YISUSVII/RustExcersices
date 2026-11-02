use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

/// Each thread increments a shared `Arc<Mutex<usize>>` `increments` times.
fn count_with_mutex(threads: usize, increments: usize) -> usize {
    let counter = Arc::new(Mutex::new(0usize));
    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..increments {
                    *counter.lock().unwrap() += 1;
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
    let total = *counter.lock().unwrap();
    total
}

/// Same as `count_with_mutex`, but lock-free using an atomic.
fn count_with_atomic(threads: usize, increments: usize) -> usize {
    let counter = AtomicUsize::new(0);
    // Scoped threads may borrow `counter` directly: no Arc needed.
    thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                for _ in 0..increments {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });
    counter.into_inner()
}

/// Sums `data` by splitting it into `threads` chunks processed in parallel.
fn parallel_sum(data: &[u64], threads: usize) -> u64 {
    if data.is_empty() {
        return 0;
    }
    let chunk_size = data.len().div_ceil(threads.max(1));
    thread::scope(|s| {
        let handles: Vec<_> = data
            .chunks(chunk_size)
            .map(|chunk| s.spawn(move || chunk.iter().sum::<u64>()))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    })
}

fn main() {
    println!("mutex counter  = {}", count_with_mutex(8, 10_000));
    println!("atomic counter = {}", count_with_atomic(8, 10_000));

    let data: Vec<u64> = (1..=1_000_000).collect();
    println!("parallel sum   = {}", parallel_sum(&data, 4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutex_counter_is_exact() {
        assert_eq!(count_with_mutex(4, 1_000), 4_000);
    }

    #[test]
    fn atomic_counter_is_exact() {
        assert_eq!(count_with_atomic(8, 5_000), 40_000);
        assert_eq!(count_with_atomic(0, 5_000), 0);
    }

    #[test]
    fn parallel_sum_matches_sequential() {
        let data: Vec<u64> = (1..=10_001).collect();
        let expected: u64 = data.iter().sum();
        for threads in [1, 3, 7, 64] {
            assert_eq!(parallel_sum(&data, threads), expected);
        }
        assert_eq!(parallel_sum(&[], 4), 0);
    }
}
