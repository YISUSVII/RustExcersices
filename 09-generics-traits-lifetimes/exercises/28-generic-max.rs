fn max_of_two<T: PartialOrd>(a: T, b: T) -> T {
    // TODO: implement
    let _ = (a, b);
    todo!("Return b if b > a, otherwise a")
}

fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    // TODO: implement
    let _ = items;
    todo!("Return None for empty slices; otherwise track the best value so far")
}

fn largest_ref<T: PartialOrd>(items: &[T]) -> Option<&T> {
    // TODO: implement
    let _ = items;
    todo!("Like largest, but keep a &T instead of copying")
}

fn max_by_key_generic<T, K, F>(items: &[T], key: F) -> Option<&T>
where
    K: Ord,
    F: Fn(&T) -> K,
{
    // TODO: implement
    let _ = (items, key);
    todo!("Compute key(item) for each item and keep the first item with the largest key")
}

fn main() {
    println!("max_of_two(3, 7) = {}", max_of_two(3, 7));
    println!("max_of_two(2.5, 1.5) = {}", max_of_two(2.5, 1.5));
    println!("max_of_two('a', 'z') = {}", max_of_two('a', 'z'));

    println!("largest([1, 5, 3]) = {:?}", largest(&[1, 5, 3]));
    println!("largest([]) = {:?}", largest::<i32>(&[]));

    let fruits = vec!["pear".to_string(), "apple".to_string(), "fig".to_string()];
    println!("largest_ref(fruits) = {:?}", largest_ref(&fruits));

    let words = ["hi", "hello", "hey"];
    println!(
        "longest word = {:?}",
        max_by_key_generic(&words, |s| s.len())
    );
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
}
