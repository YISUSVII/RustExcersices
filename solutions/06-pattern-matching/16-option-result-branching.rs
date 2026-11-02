use std::num::ParseIntError;

const USERS: [(u32, &str); 3] = [(1, "ada"), (2, "grace"), (3, "linus")];

fn parse_id(input: &str) -> Result<u32, ParseIntError> {
    input.trim().parse()
}

fn lookup(id: u32) -> Option<&'static str> {
    USERS
        .iter()
        .find(|(user_id, _)| *user_id == id)
        .map(|(_, name)| *name)
}

fn greet(input: &str) -> String {
    match parse_id(input).map(|id| (id, lookup(id))) {
        Ok((_, Some(name))) => format!("Hello, {name}!"),
        Ok((id, None)) => format!("No user with id {id}"),
        Err(e) => format!("Invalid id '{input}': {e}"),
    }
}

fn sum_valid(inputs: &[&str]) -> (u32, usize) {
    let mut sum = 0;
    let mut invalid = 0;
    for input in inputs {
        match parse_id(input) {
            Ok(n) => sum += n,
            Err(_) => invalid += 1,
        }
    }
    (sum, invalid)
}

fn main() {
    for input in ["1", "99", "abc"] {
        println!("{}", greet(input));
    }
    let (sum, invalid) = sum_valid(&["1", " 2 ", "x", "3", ""]);
    println!("sum: {sum}, invalid: {invalid}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_found_and_missing() {
        assert_eq!(greet("2"), "Hello, grace!");
        assert_eq!(greet(" 3 "), "Hello, linus!");
        assert_eq!(greet("42"), "No user with id 42");
    }

    #[test]
    fn greet_invalid() {
        assert!(greet("-1").starts_with("Invalid id '-1'"));
        assert!(greet("").starts_with("Invalid id ''"));
    }

    #[test]
    fn sum_valid_counts_failures() {
        assert_eq!(sum_valid(&["1", "2", "nope"]), (3, 1));
        assert_eq!(sum_valid(&[]), (0, 0));
    }
}
