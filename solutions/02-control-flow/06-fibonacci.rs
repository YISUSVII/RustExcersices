fn fibonacci(n: usize) -> Vec<u64> {
    let mut out = Vec::with_capacity(n);
    let (mut a, mut b) = (0_u64, 1_u64);

    for _ in 0..n {
        out.push(a);
        let next = a + b;
        a = b;
        b = next;
    }

    out
}

fn main() {
    println!("{:?}", fibonacci(7));
}
