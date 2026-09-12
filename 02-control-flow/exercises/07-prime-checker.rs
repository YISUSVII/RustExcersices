fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut d = 3_u64;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }

    true
}

fn main() {
    for n in [2, 4, 13, 25] {
        println!("{n}: {}", is_prime(n));
    }
}
