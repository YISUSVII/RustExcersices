fn parallel_sum(data: Vec<i64>, workers: usize) -> i64 {
    // TODO: partition and sum on threads
    let _ = (data, workers);
    unimplemented!("Implement parallel_sum")
}

fn main() {
    let data: Vec<i64> = (1..=1000).collect();
    println!("{}", parallel_sum(data, 4));
}
