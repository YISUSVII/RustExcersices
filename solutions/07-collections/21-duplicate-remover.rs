use std::collections::HashSet;

fn dedup_numbers(items: &[i32]) -> Vec<i32> {
    let mut seen = HashSet::new();
    items.iter().copied().filter(|n| seen.insert(*n)).collect()
}

fn dedup_words(text: &str) -> Vec<&str> {
    let mut seen = HashSet::new();
    text.split_whitespace()
        .filter(|word| seen.insert(word.to_lowercase()))
        .collect()
}

fn find_duplicates(items: &[i32]) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut dups = HashSet::new();
    for &n in items {
        if !seen.insert(n) {
            dups.insert(n);
        }
    }
    let mut result: Vec<i32> = dups.into_iter().collect();
    result.sort_unstable();
    result
}

fn dedup_in_place(items: &mut Vec<i32>) {
    let mut seen = HashSet::new();
    items.retain(|n| seen.insert(*n));
}

fn main() {
    let numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    println!("dedup_numbers: {:?}", dedup_numbers(&numbers));
    println!("dedup_words: {:?}", dedup_words("Rust is fun rust IS FUN"));
    println!("find_duplicates: {:?}", find_duplicates(&numbers));

    let mut owned = numbers.to_vec();
    dedup_in_place(&mut owned);
    println!("in place: {owned:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_first_occurrence_order() {
        assert_eq!(dedup_numbers(&[2, 2, 1, 2, 3, 1]), vec![2, 1, 3]);
        assert!(dedup_numbers(&[]).is_empty());
    }

    #[test]
    fn words_are_case_insensitive() {
        assert_eq!(dedup_words("a A b B a"), vec!["a", "b"]);
        assert_eq!(dedup_words("Hello hello"), vec!["Hello"]);
    }

    #[test]
    fn duplicates_sorted_and_unique() {
        assert_eq!(find_duplicates(&[5, 1, 5, 5, 1, 2]), vec![1, 5]);
        assert!(find_duplicates(&[1, 2, 3]).is_empty());
    }

    #[test]
    fn in_place_matches_copy() {
        let data = vec![4, 4, 3, 4, 2, 3];
        let mut v = data.clone();
        dedup_in_place(&mut v);
        assert_eq!(v, dedup_numbers(&data));
    }
}
