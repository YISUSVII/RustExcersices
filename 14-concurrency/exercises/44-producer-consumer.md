# Exercise 44 — Producer / Consumer

- **Title:** Producer / Consumer
- **Objective:** Pass messages from multiple producer threads to a consumer using `std::sync::mpsc` channels, both unbounded and bounded.
- **Difficulty:** Intermediate
- **Concepts practiced:** `mpsc::channel`, `mpsc::sync_channel`, cloning `Sender`, channel closing semantics, backpressure, `thread::scope`

## Instructions

1. Define `struct Message { producer: usize, seq: usize, value: u64 }`.
2. Implement `fn run_pipeline(producers: usize, items_per_producer: usize) -> Vec<Message>`:
   - create one channel, clone the `Sender` for each producer thread;
   - each producer sends `items_per_producer` messages with `seq` = `0..items_per_producer` and `value = producer * 1000 + seq`;
   - the calling thread consumes until the channel closes and returns all messages.
3. Implement `fn bounded_sum(producers: usize, items_per_producer: u64, bound: usize) -> u64` using `mpsc::sync_channel(bound)`. A dedicated consumer thread sums all received values; each producer sends `1..=items_per_producer`.

## Expected behavior

- `run_pipeline(5, 20).len()` → `100`.
- Messages from the *same* producer arrive in order (`seq` 0, 1, 2, …), although producers interleave.
- `bounded_sum(3, 10, 1)` → `165`.

## Optional hints

- The receiver iterator only ends when **every** `Sender` is dropped — including the original one. Forgetting `drop(tx)` makes the consumer hang forever.
- `sync_channel(bound)` blocks `send` when the buffer is full, which naturally slows fast producers (backpressure).

## Bonus challenge

Add a second stage: a "transformer" thread that receives `Message`s, doubles `value`, and forwards them to a final consumer through another channel (a 3-stage pipeline).
