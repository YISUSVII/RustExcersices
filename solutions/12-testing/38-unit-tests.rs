/// Returns true if `s` reads the same forwards and backwards,
/// ignoring case and any non-alphanumeric characters.
fn is_palindrome(s: &str) -> bool {
    let cleaned: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();
    cleaned.iter().eq(cleaned.iter().rev())
}

/// Integer division that reports errors instead of panicking.
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("division by zero".to_string());
    }
    a.checked_div(b)
        .ok_or_else(|| format!("overflow dividing {a} by {b}"))
}

/// Parses a hex color like "#ff8800" (the `#` is optional).
fn parse_rgb(hex: &str) -> Result<(u8, u8, u8), String> {
    let digits = hex.strip_prefix('#').unwrap_or(hex);
    if digits.len() != 6 || !digits.is_ascii() {
        return Err(format!("expected 6 hex digits, got '{hex}'"));
    }
    let channel = |range: std::ops::Range<usize>| {
        u8::from_str_radix(&digits[range], 16).map_err(|e| format!("invalid hex in '{hex}': {e}"))
    };
    Ok((channel(0..2)?, channel(2..4)?, channel(4..6)?))
}

/// A stack with a fixed maximum size.
#[derive(Debug)]
struct BoundedStack<T> {
    items: Vec<T>,
    capacity: usize,
}

impl<T> BoundedStack<T> {
    /// # Panics
    /// Panics if `capacity` is zero — that is a programming error, not a runtime condition.
    fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be greater than zero");
        Self {
            items: Vec::with_capacity(capacity),
            capacity,
        }
    }

    /// Pushes an item, handing it back as `Err(item)` when the stack is full.
    fn push(&mut self, item: T) -> Result<(), T> {
        if self.items.len() == self.capacity {
            return Err(item);
        }
        self.items.push(item);
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

fn main() {
    println!(
        "is_palindrome(\"A man, a plan, a canal: Panama\") = {}",
        is_palindrome("A man, a plan, a canal: Panama")
    );
    println!("divide(7, 2) = {:?}", divide(7, 2));
    println!("divide(1, 0) = {:?}", divide(1, 0));
    println!("divide(i32::MIN, -1) = {:?}", divide(i32::MIN, -1));
    println!("parse_rgb(\"#ff8800\") = {:?}", parse_rgb("#ff8800"));
    println!("parse_rgb(\"#zz0000\") = {:?}", parse_rgb("#zz0000"));

    let mut stack = BoundedStack::new(2);
    println!("push 1 -> {:?}", stack.push(1));
    println!("push 2 -> {:?}", stack.push(2));
    println!("push 3 -> {:?} (full)", stack.push(3));
    println!("peek -> {:?}, len -> {}", stack.peek(), stack.len());
    println!(
        "pop -> {:?}, pop -> {:?}, pop -> {:?}",
        stack.pop(),
        stack.pop(),
        stack.pop()
    );
    println!("is_empty -> {}", stack.is_empty());
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- is_palindrome -------------------------------------------------------

    #[test]
    fn palindrome_ignores_case_and_punctuation() {
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
        assert!(is_palindrome("Racecar"));
        assert!(!is_palindrome("hello"), "'hello' is not a palindrome");
    }

    #[test]
    fn palindrome_edge_cases() {
        assert!(is_palindrome(""), "empty string counts as a palindrome");
        assert!(is_palindrome("!!!"), "punctuation-only reduces to empty");
        assert!(is_palindrome("ÉtÉ"), "unicode letters are supported");
    }

    // --- divide --------------------------------------------------------------

    #[test]
    fn divide_happy_path_truncates_toward_zero() {
        assert_eq!(divide(7, 2), Ok(3));
        assert_eq!(divide(-7, 2), Ok(-3));
    }

    #[test]
    fn divide_reports_errors() {
        assert_eq!(divide(1, 0), Err("division by zero".to_string()));
        let err = divide(i32::MIN, -1).unwrap_err();
        assert!(err.contains("overflow"), "unexpected message: {err}");
    }

    // --- parse_rgb: Result-returning tests can use `?` -----------------------

    #[test]
    fn parse_rgb_with_and_without_hash() -> Result<(), String> {
        assert_eq!(parse_rgb("#ff8800")?, (255, 136, 0));
        assert_eq!(parse_rgb("000000")?, (0, 0, 0));
        Ok(())
    }

    #[test]
    fn parse_rgb_rejects_bad_input() {
        assert!(parse_rgb("#fff").is_err(), "too short");
        assert!(parse_rgb("#zz0000").is_err(), "not hex");
        assert!(parse_rgb("#ffé000").is_err(), "non-ascii must not panic");
    }

    // --- BoundedStack --------------------------------------------------------

    #[test]
    fn stack_is_lifo_and_bounded() -> Result<(), i32> {
        let mut stack = BoundedStack::new(2);
        assert!(stack.is_empty());
        stack.push(1)?;
        stack.push(2)?;
        assert_eq!(stack.push(3), Err(3), "full stack must hand the item back");
        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "capacity must be greater than zero")]
    fn stack_with_zero_capacity_panics() {
        let _ = BoundedStack::<u8>::new(0);
    }
}
