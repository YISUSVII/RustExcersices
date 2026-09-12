fn sum_and_count(nums: &[i32]) -> (i32, usize) {
    nums.iter().fold((0, 0), |(sum, count), n| (sum + n, count + 1))
}

fn main() {
    println!("{:?}", sum_and_count(&[1, 2, 3]));
    println!("{:?}", sum_and_count(&[]));
}
