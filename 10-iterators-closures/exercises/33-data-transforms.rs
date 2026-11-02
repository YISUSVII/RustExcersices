use std::collections::BTreeMap;

fn squares_of_evens(nums: &[i32]) -> Vec<i32> {
    // TODO: implement with filter + map + collect
    let _ = nums;
    todo!("Keep evens, square them")
}

fn parse_numbers(inputs: &[&str]) -> Vec<i32> {
    // TODO: implement with filter_map
    let _ = inputs;
    todo!("Trim, parse, skip failures")
}

fn try_parse_all(inputs: &[&str]) -> Result<Vec<i32>, String> {
    // TODO: implement by collecting into Result<Vec<_>, _>
    let _ = inputs;
    todo!("Fail with \"invalid number: '<input>'\" on the first bad input")
}

fn dot_product(a: &[f64], b: &[f64]) -> Option<f64> {
    // TODO: implement with zip
    let _ = (a, b);
    todo!("None if lengths differ, otherwise sum of a[i] * b[i]")
}

fn running_total(nums: &[i32]) -> Vec<i32> {
    // TODO: implement with scan
    let _ = nums;
    todo!("Prefix sums")
}

fn index_of_max(nums: &[i32]) -> Option<usize> {
    // TODO: implement with enumerate
    let _ = nums;
    todo!("Index of the first maximum value")
}

fn scale_by(nums: &[i32], factor: i32) -> Vec<i32> {
    // TODO: implement; the closure should capture `factor`
    let _ = (nums, factor);
    todo!("Multiply every number by factor")
}

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // TODO: return a `move` closure that adds n
    move |x| {
        let _ = (x, n);
        todo!("x + n")
    }
}

fn char_frequencies(text: &str) -> BTreeMap<char, usize> {
    // TODO: implement with fold
    let _ = text;
    todo!("Count lowercase alphabetic characters")
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
    println!("running_total: {:?}", running_total(&[1, 2, 3]));
    println!("index_of_max: {:?}", index_of_max(&[3, 9, 2, 9]));
    println!("scale_by 3: {:?}", scale_by(&[1, 2, 3], 3));

    let add_ten = make_adder(10);
    println!("make_adder(10)(5): {}", add_ten(5));
    println!("char_frequencies: {:?}", char_frequencies("Hello"));
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
    }

    #[test]
    fn frequencies_are_case_insensitive() {
        let f = char_frequencies("Hello, hi!");
        assert_eq!(f.get(&'h'), Some(&2));
        assert_eq!(f.get(&'l'), Some(&2));
        assert_eq!(f.get(&','), None);
    }
}
