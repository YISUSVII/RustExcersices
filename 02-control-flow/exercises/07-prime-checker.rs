fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    let limit = (n as f64).sqrt() as u64;
    for d in 2..=limit {
        if n % d == 0 {
            return false;
        }
    }
    true
}

fn main() {
    for n in [2, 4, 13, 25] {
        println!("{n}: {}", is_prime(n));
    }
}
