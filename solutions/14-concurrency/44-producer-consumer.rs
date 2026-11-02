use std::sync::mpsc;
use std::thread;

#[derive(Debug, Clone, PartialEq)]
struct Message {
    producer: usize,
    seq: usize,
    value: u64,
}

/// Spawns `producers` threads, each sending `items_per_producer` messages
/// over a single channel. The calling thread is the consumer.
fn run_pipeline(producers: usize, items_per_producer: usize) -> Vec<Message> {
    let (tx, rx) = mpsc::channel();

    for id in 0..producers {
        let tx = tx.clone();
        thread::spawn(move || {
            for seq in 0..items_per_producer {
                let msg = Message {
                    producer: id,
                    seq,
                    value: (id * 1000 + seq) as u64,
                };
                if tx.send(msg).is_err() {
                    break; // consumer hung up
                }
            }
        });
    }
    // Drop the original sender so the receiver loop ends once all producers finish.
    drop(tx);

    rx.into_iter().collect()
}

/// Bounded pipeline: producers block when the buffer (capacity `bound`) is full.
/// Returns the sum of all values computed by a dedicated consumer thread.
fn bounded_sum(producers: usize, items_per_producer: u64, bound: usize) -> u64 {
    let (tx, rx) = mpsc::sync_channel::<u64>(bound);
    let consumer = thread::spawn(move || rx.iter().sum::<u64>());

    thread::scope(|s| {
        for _ in 0..producers {
            let tx = tx.clone();
            s.spawn(move || {
                for v in 1..=items_per_producer {
                    tx.send(v).unwrap();
                }
            });
        }
    });
    drop(tx);
    consumer.join().unwrap()
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
