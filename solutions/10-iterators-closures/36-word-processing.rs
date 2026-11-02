use std::collections::BTreeMap;

fn normalize_words(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != '\''))
        .map(|w| w.trim_matches('\''))
        .filter(|w| !w.is_empty())
        .map(str::to_lowercase)
}

fn word_counts(text: &str) -> BTreeMap<String, usize> {
    normalize_words(text).fold(BTreeMap::new(), |mut counts, w| {
        *counts.entry(w).or_insert(0) += 1;
        counts
    })
}

fn top_n(text: &str, n: usize) -> Vec<(String, usize)> {
    let mut counts: Vec<(String, usize)> = word_counts(text).into_iter().collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    counts.truncate(n);
    counts
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

fn long_words(text: &str, min_len: usize) -> impl Iterator<Item = &str> + '_ {
    text.split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(move |w| w.chars().count() >= min_len)
}

fn bigrams(text: &str) -> Vec<(String, String)> {
    let words: Vec<String> = normalize_words(text).collect();
    words
        .windows(2)
        .map(|pair| (pair[0].clone(), pair[1].clone()))
        .collect()
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
        let rest = self.rest.trim_start();
        if rest.is_empty() {
            self.rest = rest;
            return None;
        }
        // Terminators are ASCII, so `i + 1` is always a char boundary.
        let end = rest.find(['.', '!', '?']).map_or(rest.len(), |i| i + 1);
        let (sentence, remaining) = rest.split_at(end);
        self.rest = remaining;
        Some(sentence.trim_end())
    }
}

struct Chunked<I> {
    inner: I,
    size: usize,
}

impl<I: Iterator> Iterator for Chunked<I> {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk: Vec<I::Item> = self.inner.by_ref().take(self.size).collect();
        (!chunk.is_empty()).then_some(chunk)
    }
}

trait ChunkedExt: Iterator + Sized {
    fn chunked(self, size: usize) -> Chunked<Self>;
}

impl<I: Iterator> ChunkedExt for I {
    fn chunked(self, size: usize) -> Chunked<Self> {
        Chunked {
            inner: self,
            size: size.max(1),
        }
    }
}

struct Interleave<A, B> {
    a: A,
    b: B,
    take_a: bool,
}

impl<A, B> Iterator for Interleave<A, B>
where
    A: Iterator,
    B: Iterator<Item = A::Item>,
{
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let first_try = self.take_a;
        self.take_a = !self.take_a;
        if first_try {
            self.a.next().or_else(|| self.b.next())
        } else {
            self.b.next().or_else(|| self.a.next())
        }
    }
}

fn interleave<A, B>(a: A, b: B) -> Interleave<A::IntoIter, B::IntoIter>
where
    A: IntoIterator,
    B: IntoIterator<Item = A::Item>,
{
    Interleave {
        a: a.into_iter(),
        b: b.into_iter(),
        take_a: true,
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

    let mixed: Vec<i32> = interleave(vec![1, 3, 5, 7], vec![2, 4]).collect();
    println!("interleave: {mixed:?}");
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
    fn custom_adaptors() {
        let chunks: Vec<Vec<i32>> = (1..=7).chunked(3).collect();
        assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
        assert_eq!((1..=2).chunked(0).count(), 2);
        let mixed: Vec<i32> = interleave(1..=2, 10..=13).collect();
        assert_eq!(mixed, vec![1, 10, 2, 11, 12, 13]);
    }
}
