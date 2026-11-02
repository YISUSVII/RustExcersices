fn max_of_two<T: PartialOrd>(a: T, b: T) -> T {
    if b > a {
        b
    } else {
        a
    }
}

fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let (&first, rest) = items.split_first()?;
    Some(rest.iter().fold(first, |best, &x| max_of_two(best, x)))
}

fn largest_ref<T: PartialOrd>(items: &[T]) -> Option<&T> {
    let (first, rest) = items.split_first()?;
    Some(rest.iter().fold(first, |best, x| max_of_two(best, x)))
}

fn max_by_key_generic<T, K, F>(items: &[T], key: F) -> Option<&T>
where
    K: Ord,
    F: Fn(&T) -> K,
{
    let mut iter = items.iter();
    let first = iter.next()?;
    let mut best = (first, key(first));
    for item in iter {
        let k = key(item);
        // Strictly greater keeps the first item on ties.
        if k > best.1 {
            best = (item, k);
        }
    }
    Some(best.0)
}

fn min_max<T: PartialOrd + Copy>(items: &[T]) -> Option<(T, T)> {
    let (&first, rest) = items.split_first()?;
    Some(rest.iter().fold((first, first), |(lo, hi), &x| {
        (if x < lo { x } else { lo }, if x > hi { x } else { hi })
    }))
}

fn main() {
    println!("max_of_two(3, 7) = {}", max_of_two(3, 7));
    println!("max_of_two(2.5, 1.5) = {}", max_of_two(2.5, 1.5));
    println!("max_of_two('a', 'z') = {}", max_of_two('a', 'z'));

    println!("largest([1, 5, 3]) = {:?}", largest(&[1, 5, 3]));
    println!("largest([]) = {:?}", largest::<i32>(&[]));
    println!(
        "largest([0.5, -1.0, 9.75]) = {:?}",
        largest(&[0.5, -1.0, 9.75])
    );

    let fruits = vec!["pear".to_string(), "apple".to_string(), "fig".to_string()];
    println!("largest_ref(fruits) = {:?}", largest_ref(&fruits));

    let words = ["hi", "hello", "hey"];
    println!(
        "longest word = {:?}",
        max_by_key_generic(&words, |s| s.len())
    );

    println!("min_max([4, -2, 8, 0]) = {:?}", min_max(&[4, -2, 8, 0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_of_two_works_for_many_types() {
        assert_eq!(max_of_two(3, 7), 7);
        assert_eq!(max_of_two('z', 'a'), 'z');
        assert_eq!(max_of_two(1.5, 2.5), 2.5);
    }

    #[test]
    fn largest_handles_empty_and_non_empty() {
        assert_eq!(largest(&[1, 5, 3]), Some(5));
        assert_eq!(largest::<u8>(&[]), None);
    }

    #[test]
    fn largest_ref_works_with_strings() {
        let v = vec!["pear".to_string(), "apple".to_string()];
        assert_eq!(largest_ref(&v).map(String::as_str), Some("pear"));
    }

    #[test]
    fn max_by_key_returns_first_on_tie() {
        let words = ["abc", "xyz", "a"];
        assert_eq!(max_by_key_generic(&words, |s| s.len()), Some(&"abc"));
    }

    #[test]
    fn min_max_single_pass() {
        assert_eq!(min_max(&[4, -2, 8, 0]), Some((-2, 8)));
        assert_eq!(min_max::<i32>(&[]), None);
    }
}
