use std::collections::HashMap;

fn word_frequencies(text: &str) -> HashMap<String, usize> {
    let mut freqs = HashMap::new();
    for raw in text.split_whitespace() {
        let word = raw.trim_matches(|c: char| !c.is_alphanumeric());
        if word.is_empty() {
            continue;
        }
        *freqs.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    freqs
}

fn top_n(freqs: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut pairs: Vec<(String, usize)> = freqs
        .iter()
        .map(|(word, count)| (word.clone(), *count))
        .collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    pairs.truncate(n);
    pairs
}

fn main() {
    let text = "The Rust book. Rust is fast; the borrow checker is the friend.";
    let freqs = word_frequencies(text);
    for (word, count) in top_n(&freqs, 3) {
        println!("{word}: {count}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_case_insensitively_and_strips_punctuation() {
        let f = word_frequencies("Hello, hello! HELLO? world");
        assert_eq!(f.get("hello"), Some(&3));
        assert_eq!(f.get("world"), Some(&1));
        assert_eq!(f.len(), 2);
    }

    #[test]
    fn ignores_pure_punctuation() {
        let f = word_frequencies("  -- ... !!  ");
        assert!(f.is_empty());
    }

    #[test]
    fn top_n_sorts_and_breaks_ties() {
        let f = word_frequencies("b a c b a d");
        assert_eq!(
            top_n(&f, 3),
            vec![
                ("a".to_string(), 2),
                ("b".to_string(), 2),
                ("c".to_string(), 1)
            ]
        );
        assert_eq!(top_n(&f, 10).len(), 4);
    }
}
