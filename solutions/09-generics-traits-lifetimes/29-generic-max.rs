fn max_of<T: Ord>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

fn main() {
    println!("{}", max_of(3, 10));
    println!("{}", max_of('a', 'z'));
}
