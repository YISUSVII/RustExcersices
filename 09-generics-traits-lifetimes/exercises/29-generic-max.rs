fn max_of<T: Ord>(a: T, b: T) -> T {
    let _ = (a, b);
    unimplemented!("Implement max_of")
}

fn main() {
    println!("{}", max_of(3, 10));
    println!("{}", max_of('a', 'z'));
}
