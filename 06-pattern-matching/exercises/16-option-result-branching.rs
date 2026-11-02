use std::num::ParseIntError;

const USERS: [(u32, &str); 3] = [(1, "ada"), (2, "grace"), (3, "linus")];

fn parse_id(input: &str) -> Result<u32, ParseIntError> {
    // TODO: implement
    let _ = input;
    todo!("Trim the input and parse it into a u32")
}

fn lookup(id: u32) -> Option<&'static str> {
    // TODO: implement
    let _ = (id, USERS);
    todo!("Find the name for id in USERS")
}

fn greet(input: &str) -> String {
    // TODO: implement
    let _ = input;
    todo!("match on Ok(Some(name)), Ok(None) and Err(e)")
}

fn sum_valid(inputs: &[&str]) -> (u32, usize) {
    // TODO: implement
    let _ = inputs;
    todo!("Return (sum of valid ids, number of invalid inputs)")
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
