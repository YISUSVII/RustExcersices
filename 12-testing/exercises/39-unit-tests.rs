fn add(a: i32, b: i32) -> i32 {
    let _ = (a, b);
    unimplemented!("Implement add")
}

fn saturating_sub_u8(a: u8, b: u8) -> u8 {
    // TODO: if b > a return 0 else a - b
    let _ = (a, b);
    unimplemented!("Implement saturating_sub_u8")
}

fn is_palindrome(s: &str) -> bool {
    // TODO: ignore case; compare to reversed chars
    let _ = s;
    unimplemented!("Implement is_palindrome")
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
