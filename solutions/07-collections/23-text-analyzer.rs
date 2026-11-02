use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
struct TextStats {
    chars: usize,
    words: usize,
    lines: usize,
    sentences: usize,
    longest_word: Option<String>,
    avg_word_len: f64,
}

fn clean_words(text: &str) -> impl Iterator<Item = &str> {
    text.split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| !w.is_empty())
}

fn analyze(text: &str) -> TextStats {
    let words: Vec<&str> = clean_words(text).collect();
    let total_len: usize = words.iter().map(|w| w.chars().count()).sum();

    // reduce keeps the first word on ties (max_by_key would keep the last).
    let longest_word = words
        .iter()
        .copied()
        .reduce(|best, w| {
            if w.chars().count() > best.chars().count() {
                w
            } else {
                best
            }
        })
        .map(str::to_string);

    let avg_word_len = if words.is_empty() {
        0.0
    } else {
        total_len as f64 / words.len() as f64
    };

    TextStats {
        chars: text.chars().count(),
        words: text.split_whitespace().count(),
        lines: text.lines().count(),
        sentences: text
            .chars()
            .filter(|c| matches!(c, '.' | '!' | '?'))
            .count(),
        longest_word,
        avg_word_len,
    }
}

fn letter_histogram(text: &str) -> BTreeMap<char, usize> {
    let mut hist = BTreeMap::new();
    for c in text.chars().filter(|c| c.is_alphabetic()) {
        for lower in c.to_lowercase() {
            *hist.entry(lower).or_insert(0) += 1;
        }
    }
    hist
}

fn capitalize_words(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    let text = "Rust makes café code safe.\n¿Tienes tiempo para Rust?";
    let mut stats = analyze(text);
    stats.avg_word_len = (stats.avg_word_len * 100.0).round() / 100.0;
    println!("{stats:?}");

    let mut top: Vec<(char, usize)> = letter_histogram(text).into_iter().collect();
    top.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    top.truncate(3);
    println!("top letters: {top:?}");

    println!("{}", capitalize_words("rust makes   café code safe"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_unicode_chars_not_bytes() {
        let s = analyze("café");
        assert_eq!(s.chars, 4);
        assert_eq!("café".len(), 5);
    }

    #[test]
    fn basic_stats() {
        let s = analyze("Hi there. How are you?\nFine!");
        assert_eq!(s.words, 6);
        assert_eq!(s.lines, 2);
        assert_eq!(s.sentences, 3);
        assert_eq!(s.longest_word.as_deref(), Some("there"));
    }

    #[test]
    fn empty_text() {
        let s = analyze("");
        assert_eq!(s.words, 0);
        assert_eq!(s.longest_word, None);
        assert_eq!(s.avg_word_len, 0.0);
    }

    #[test]
    fn histogram_and_capitalize() {
        let h = letter_histogram("AaB!");
        assert_eq!(h.get(&'a'), Some(&2));
        assert_eq!(h.get(&'b'), Some(&1));
        assert_eq!(h.len(), 2);
        assert_eq!(capitalize_words("élan  vital"), "Élan Vital");
    }
}
