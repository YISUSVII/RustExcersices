use std::collections::BTreeMap;

fn normalize_words(text: &str) -> impl Iterator<Item = String> + '_ {
    // TODO: implement: split_whitespace, trim punctuation (keep inner '), lowercase, skip empty
    text.split_whitespace().map(|w| {
        let _ = w;
        todo!("Normalize each word")
    })
}

fn word_counts(text: &str) -> BTreeMap<String, usize> {
    // TODO: implement using normalize_words + fold
    let _ = text;
    todo!("Count each normalized word")
}

fn top_n(text: &str, n: usize) -> Vec<(String, usize)> {
    // TODO: implement
    let _ = (text, n);
    todo!("Sort by count desc, then word asc, and keep n")
}

fn capitalize_words(text: &str) -> String {
    // TODO: implement
    let _ = text;
    todo!("Uppercase the first char of every word")
}

fn long_words(text: &str, min_len: usize) -> impl Iterator<Item = &str> + '_ {
    // TODO: implement without allocating
    let _ = min_len;
    text.split_whitespace()
        .filter(|_| todo!("Keep words with at least min_len chars"))
}

fn bigrams(text: &str) -> Vec<(String, String)> {
    // TODO: implement with windows(2)
    let _ = text;
    todo!("Pairs of consecutive normalized words")
}

struct Sentences<'a> {
    rest: &'a str,
}

fn sentences(text: &str) -> Sentences<'_> {
    Sentences { rest: text }
}

impl<'a> Iterator for Sentences<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: implement
        let _ = self.rest;
        todo!("Yield the next trimmed sentence ending in '.', '!' or '?'")
    }
}

struct Chunked<I> {
    inner: I,
    size: usize,
}

impl<I: Iterator> Iterator for Chunked<I> {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: implement
        let _ = (&mut self.inner, self.size);
        todo!("Collect up to size items; None when nothing is left")
    }
}

trait ChunkedExt: Iterator + Sized {
    fn chunked(self, size: usize) -> Chunked<Self>;
}

impl<I: Iterator> ChunkedExt for I {
    fn chunked(self, size: usize) -> Chunked<Self> {
        // TODO: treat size 0 as 1
        Chunked { inner: self, size }
    }
}

fn main() {
    let text = "The quick brown fox. The lazy dog sleeps! Does the fox care? No.";

    let words: Vec<String> = normalize_words(text).collect();
    println!("normalized: {words:?}");
    println!("top 3: {:?}", top_n(text, 3));
    println!("capitalized: {}", capitalize_words("hello big world"));
    println!("long words: {:?}", long_words(text, 5).collect::<Vec<_>>());
    println!("bigrams: {:?}", bigrams("one two three"));

    for (i, s) in sentences(text).enumerate() {
        println!("sentence {}: {s}", i + 1);
    }

    for chunk in words.iter().chunked(4) {
        println!("chunk: {chunk:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizing_and_counting() {
        let words: Vec<String> = normalize_words("Hello, world! It's   fine.").collect();
        assert_eq!(words, vec!["hello", "world", "it's", "fine"]);
        assert_eq!(
            top_n("a b a c b a", 2),
            vec![("a".to_string(), 3), ("b".to_string(), 2)]
        );
    }

    #[test]
    fn borrowing_and_formatting_helpers() {
        assert_eq!(capitalize_words("hello big  world"), "Hello Big World");
        let found: Vec<&str> = long_words("a tiny elephant!", 4).collect();
        assert_eq!(found, vec!["tiny", "elephant"]);
        assert_eq!(
            bigrams("one two three"),
            vec![
                ("one".to_string(), "two".to_string()),
                ("two".to_string(), "three".to_string())
            ]
        );
    }

    #[test]
    fn sentences_iterator() {
        let got: Vec<&str> = sentences("Hi there. How are you?  Great! trailing").collect();
        assert_eq!(got, vec!["Hi there.", "How are you?", "Great!", "trailing"]);
        assert_eq!(sentences("   ").next(), None);
    }

    #[test]
    fn chunked_adaptor() {
        let chunks: Vec<Vec<i32>> = (1..=7).chunked(3).collect();
        assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
        assert_eq!((1..=2).chunked(0).count(), 2);
    }
}
