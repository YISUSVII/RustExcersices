fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if b.len() > a.len() {
        b
    } else {
        a
    }
}

// One reference input => the elision rules give the output the same lifetime.
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// `prefix` gets its own (elided) lifetime because the result never borrows from it.
fn longest_with_prefix<'a>(words: &[&'a str], prefix: &str) -> Option<&'a str> {
    words
        .iter()
        .copied()
        .filter(|w| w.starts_with(prefix))
        .fold(None, |best, w| match best {
            Some(b) if b.len() >= w.len() => Some(b),
            _ => Some(w),
        })
}

fn split_key_value(line: &str) -> Option<(&str, &str)> {
    let (key, value) = line.split_once('=')?;
    let (key, value) = (key.trim(), value.trim());
    if key.is_empty() {
        None
    } else {
        Some((key, value))
    }
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn remaining(&self) -> &'a str {
        self.input[self.pos..].trim_start()
    }

    // Returning `&'a str` (not `&str`) ties the token to the input, not to `&mut self`.
    fn next_token(&mut self) -> Option<&'a str> {
        let rest = self.remaining();
        if rest.is_empty() {
            self.pos = self.input.len();
            return None;
        }
        let start = self.input.len() - rest.len();
        let len = rest.find(char::is_whitespace).unwrap_or(rest.len());
        self.pos = start + len;
        Some(&self.input[start..start + len])
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn main() {
    println!("longest = {}", longest("apple", "fig"));
    println!("first_word = {:?}", first_word("  hello world"));
    println!(
        "longest_with_prefix = {:?}",
        longest_with_prefix(&["car", "cartoon", "dog"], "car")
    );

    let source = String::from("let x = 5");
    let mut parser = Parser::new(&source);
    let mut tokens: Vec<&str> = Vec::new();
    while let Some(tok) = parser.next_token() {
        tokens.push(tok);
        if tokens.len() == 2 {
            println!("after 2 tokens, remaining = {:?}", parser.remaining());
        }
    }
    println!("tokens = {tokens:?}");

    for (i, tok) in Parser::new("  fn main ( )  ").enumerate() {
        println!("iter token {i}: {tok}");
    }

    for line in ["name = Ferris", "  age=7 ", "no equals"] {
        println!("{line:?} -> {:?}", split_key_value(line));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_and_first_word() {
        assert_eq!(longest("apple", "fig"), "apple");
        assert_eq!(longest("ab", "cd"), "ab");
        assert_eq!(first_word("  hello world"), "hello");
        assert_eq!(first_word("   "), "");
    }

    #[test]
    fn prefix_search_borrows_from_words() {
        let words = vec!["car", "cartoon", "dog"];
        let found;
        {
            let prefix = String::from("car");
            found = longest_with_prefix(&words, &prefix);
        } // prefix dropped here; `found` is still valid.
        assert_eq!(found, Some("cartoon"));
        assert_eq!(longest_with_prefix(&words, "z"), None);
    }

    #[test]
    fn parser_yields_tokens_that_outlive_the_parser() {
        let input = String::from("let x = 5");
        let tokens: Vec<&str> = {
            let parser = Parser::new(&input);
            parser.collect()
        };
        assert_eq!(tokens, vec!["let", "x", "=", "5"]);
    }

    #[test]
    fn key_value_splitting() {
        assert_eq!(split_key_value("name = Ferris"), Some(("name", "Ferris")));
        assert_eq!(split_key_value("no equals"), None);
        assert_eq!(split_key_value(" = value"), None);
    }
}
