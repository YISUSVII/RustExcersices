use std::collections::BTreeMap;

fn squares_of_evens(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .collect()
}

fn parse_numbers(inputs: &[&str]) -> Vec<i32> {
    inputs
        .iter()
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

fn try_parse_all(inputs: &[&str]) -> Result<Vec<i32>, String> {
    inputs
        .iter()
        .map(|s| {
            s.trim()
                .parse::<i32>()
                .map_err(|_| format!("invalid number: '{s}'"))
        })
        .collect()
}

fn dot_product(a: &[f64], b: &[f64]) -> Option<f64> {
    (a.len() == b.len()).then(|| a.iter().zip(b).map(|(x, y)| x * y).sum())
}

fn running_total(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .scan(0, |acc, &n| {
            *acc += n;
            Some(*acc)
        })
        .collect()
}

fn index_of_max(nums: &[i32]) -> Option<usize> {
    nums.iter()
        .enumerate()
        .fold(None, |best: Option<(usize, i32)>, (i, &n)| match best {
            Some((_, b)) if b >= n => best,
            _ => Some((i, n)),
        })
        .map(|(i, _)| i)
}

fn scale_by(nums: &[i32], factor: i32) -> Vec<i32> {
    nums.iter().map(|n| n * factor).collect()
}

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn char_frequencies(text: &str) -> BTreeMap<char, usize> {
    text.chars()
        .filter(|c| c.is_alphabetic())
        .flat_map(char::to_lowercase)
        .fold(BTreeMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
}

fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |x| g(f(x))
}

fn main() {
    println!("squares_of_evens: {:?}", squares_of_evens(&[1, 2, 3, 4]));
    println!(
        "parse_numbers: {:?}",
        parse_numbers(&["1", " 2 ", "x", "3"])
    );
    println!("try_parse_all ok: {:?}", try_parse_all(&["1", "2"]));
    println!("try_parse_all err: {:?}", try_parse_all(&["1", "x"]));
    println!("dot_product: {:?}", dot_product(&[1.0, 2.0], &[3.0, 4.0]));
    println!("dot_product mismatch: {:?}", dot_product(&[1.0], &[]));
    println!("running_total: {:?}", running_total(&[1, 2, 3]));
    println!("index_of_max: {:?}", index_of_max(&[3, 9, 2, 9]));
    println!("scale_by 3: {:?}", scale_by(&[1, 2, 3], 3));

    let add_ten = make_adder(10);
    println!("make_adder(10)(5): {}", add_ten(5));
    println!("char_frequencies: {:?}", char_frequencies("Hello"));

    let pipeline = compose(make_adder(1), |x| x * 2);
    println!("compose(+1, *2)(4): {}", pipeline(4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filtering_and_parsing() {
        assert_eq!(squares_of_evens(&[1, 2, 3, 4]), vec![4, 16]);
        assert_eq!(parse_numbers(&["1", " 2 ", "x", "3"]), vec![1, 2, 3]);
        assert_eq!(
            try_parse_all(&["1", "x"]),
            Err("invalid number: 'x'".into())
        );
    }

    #[test]
    fn numeric_combinators() {
        assert_eq!(dot_product(&[1.0, 2.0], &[3.0, 4.0]), Some(11.0));
        assert_eq!(dot_product(&[1.0], &[1.0, 2.0]), None);
        assert_eq!(running_total(&[1, 2, 3]), vec![1, 3, 6]);
        assert_eq!(index_of_max(&[3, 9, 2, 9]), Some(1));
        assert_eq!(index_of_max(&[]), None);
    }

    #[test]
    fn closures_capture_environment() {
        assert_eq!(scale_by(&[1, -2], 3), vec![3, -6]);
        assert_eq!(make_adder(10)(5), 15);
        assert_eq!(compose(make_adder(1), |x| x * 2)(4), 10);
    }

    #[test]
    fn frequencies_are_case_insensitive() {
        let f = char_frequencies("Hello, hi!");
        assert_eq!(f.get(&'h'), Some(&2));
        assert_eq!(f.get(&'l'), Some(&2));
        assert_eq!(f.get(&','), None);
    }
}
