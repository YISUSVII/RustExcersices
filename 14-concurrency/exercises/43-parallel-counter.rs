#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};
#[allow(unused_imports)]
use std::sync::{Arc, Mutex};
#[allow(unused_imports)]
use std::thread;

fn count_with_mutex(threads: usize, increments: usize) -> usize {
    // TODO: implement
    let _ = (threads, increments);
    todo!("Spawn threads sharing an Arc<Mutex<usize>>, join them, return the total")
}

fn count_with_atomic(threads: usize, increments: usize) -> usize {
    // TODO: implement
    let _ = (threads, increments);
    todo!("Use thread::scope and AtomicUsize::fetch_add(1, Ordering::Relaxed)")
}

fn parallel_sum(data: &[u64], threads: usize) -> u64 {
    // TODO: implement
    let _ = (data, threads);
    todo!("Split data with chunks(), sum each chunk in a scoped thread, add the results")
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
