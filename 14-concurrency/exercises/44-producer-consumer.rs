#![allow(dead_code)]
#[allow(unused_imports)]
use std::sync::mpsc;
#[allow(unused_imports)]
use std::thread;

#[derive(Debug, Clone, PartialEq)]
struct Message {
    producer: usize,
    seq: usize,
    value: u64,
}

fn run_pipeline(producers: usize, items_per_producer: usize) -> Vec<Message> {
    // TODO: implement
    let _ = (producers, items_per_producer);
    todo!("Create a channel, clone tx per producer, drop the original tx, collect rx")
}

fn bounded_sum(producers: usize, items_per_producer: u64, bound: usize) -> u64 {
    // TODO: implement
    let _ = (producers, items_per_producer, bound);
    todo!("Use mpsc::sync_channel(bound) and a consumer thread that sums rx.iter()")
}

fn main() {
    let messages = run_pipeline(3, 4);
    for m in &messages {
        println!("producer {} -> seq {} value {}", m.producer, m.seq, m.value);
    }
    println!("received {} messages", messages.len());

    println!("bounded sum = {}", bounded_sum(4, 100, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receives_every_message() {
        let msgs = run_pipeline(5, 20);
        assert_eq!(msgs.len(), 100);
    }

    #[test]
    fn per_producer_order_is_preserved() {
        let msgs = run_pipeline(4, 50);
        for p in 0..4 {
            let seqs: Vec<usize> = msgs.iter().filter(|m| m.producer == p).map(|m| m.seq).collect();
            assert_eq!(seqs, (0..50).collect::<Vec<_>>());
        }
    }

    #[test]
    fn bounded_channel_sums_correctly() {
        assert_eq!(bounded_sum(3, 10, 1), 3 * 55);
        assert_eq!(bounded_sum(0, 10, 1), 0);
    }
}
