fn is_prime(n: u64) -> bool {
    // TODO: return true only for prime numbers
    let _ = n;
    unimplemented!("Implement is_prime")
}

fn main() {
    for n in [2, 4, 13, 25] {
        println!("{n}: {}", is_prime(n));
    }
}
