fn fibonacci(n: usize) -> Vec<u64> {
    let mut out = Vec::with_capacity(n);
    let (mut a, mut b) = (0_u64, 1_u64);

    for i in 0..n {
        if i == 0 {
            out.push(0);
        } else if i == 1 {
            out.push(1);
        } else {
            let next = a + b;
            out.push(next);
            a = b;
            b = next;
        }
    }

    out
}

fn main() {
    println!("{:?}", fibonacci(7));
}
