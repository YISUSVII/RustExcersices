fn squares_of_evens(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .copied()
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .collect()
}

fn main() {
    println!("{:?}", squares_of_evens(&[1, 2, 3, 4]));
}
