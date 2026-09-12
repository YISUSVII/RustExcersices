fn parallel_sum(data: Vec<i64>, workers: usize) -> i64 {
    if data.is_empty() {
        return 0;
    }
    let workers = workers.max(1).min(data.len());
    let chunk_size = (data.len() + workers - 1) / workers;
    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for chunk in data.chunks(chunk_size) {
            handles.push(scope.spawn(move || chunk.iter().sum::<i64>()));
        }
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    })
}

fn main() {
    let data: Vec<i64> = (1..=1000).collect();
    println!("{}", parallel_sum(data, 4));
}
