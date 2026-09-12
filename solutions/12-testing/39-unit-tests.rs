fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn saturating_sub_u8(a: u8, b: u8) -> u8 {
    a.saturating_sub(b)
}

fn is_palindrome(s: &str) -> bool {
    let lower: String = s.to_lowercase();
    lower.chars().eq(lower.chars().rev())
}

fn main() {
    println!("{}", add(2, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn saturating_sub_bottom() {
        assert_eq!(saturating_sub_u8(3, 10), 0);
        assert_eq!(saturating_sub_u8(10, 3), 7);
    }

    #[test]
    fn palindrome_cases() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("RaceCar"));
        assert!(is_palindrome(""));
        assert!(!is_palindrome("rust"));
    }
}
