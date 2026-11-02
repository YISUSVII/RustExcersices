fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    // TODO: implement
    let _ = (a, b);
    todo!("Return the longer string (a on ties)")
}

// TODO: explain in a comment why no lifetime annotation is needed here.
fn first_word(s: &str) -> &str {
    // TODO: implement
    let _ = s;
    todo!("Skip leading whitespace and return the first word, or an empty str")
}

fn longest_with_prefix<'a>(words: &[&'a str], prefix: &str) -> Option<&'a str> {
    // TODO: implement
    let _ = (words, prefix);
    todo!("Keep the longest word that starts with prefix")
}

fn split_key_value(line: &str) -> Option<(&str, &str)> {
    // TODO: implement
    let _ = line;
    todo!("Use split_once('=') and trim both sides")
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
        // TODO: implement
        let _ = (self.input, self.pos);
        todo!("Return the unparsed part of input with leading whitespace trimmed")
    }

    fn next_token(&mut self) -> Option<&'a str> {
        // TODO: implement
        todo!("Return the next whitespace-separated token and advance pos")
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
        }
        assert_eq!(found, Some("cartoon"));
        assert_eq!(longest_with_prefix(&words, "z"), None);
    }

    #[test]
    fn parser_tokens_outlive_the_parser() {
        let input = String::from("let x = 5");
        let tokens: Vec<&str> = {
            let mut parser = Parser::new(&input);
            let mut out = Vec::new();
            while let Some(t) = parser.next_token() {
                out.push(t);
            }
            out
        };
        assert_eq!(tokens, vec!["let", "x", "=", "5"]);
    }

    #[test]
    fn key_value_splitting() {
        assert_eq!(split_key_value("name = Ferris"), Some(("name", "Ferris")));
        assert_eq!(split_key_value("no equals"), None);
    }
}
