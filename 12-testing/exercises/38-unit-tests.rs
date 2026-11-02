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

// The functions above are already implemented. Your job is to write the tests.
// Run them with:
//   rustc --edition 2021 --test 38-unit-tests.rs -o unit_tests && ./unit_tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindrome_ignores_case_and_punctuation() {
        // TODO: assert that "A man, a plan, a canal: Panama" and "Racecar" are palindromes
        //       and that "hello" is not (add a custom failure message).
        let _ = is_palindrome;
        todo!("write assertions")
    }

    #[test]
    fn palindrome_edge_cases() {
        // TODO: empty string, punctuation-only string, unicode letters
        todo!("write assertions")
    }

    #[test]
    fn divide_happy_path_truncates_toward_zero() {
        // TODO: 7 / 2 and -7 / 2
        let _ = divide;
        todo!("write assertions")
    }

    #[test]
    fn divide_reports_errors() {
        // TODO: division by zero and i32::MIN / -1 (check the message contains "overflow")
        todo!("write assertions")
    }

    #[test]
    fn parse_rgb_with_and_without_hash() -> Result<(), String> {
        // TODO: use `?` instead of unwrap, e.g. assert_eq!(parse_rgb("#ff8800")?, (255, 136, 0));
        let _ = parse_rgb;
        todo!("write assertions, then return Ok(())")
    }

    #[test]
    fn parse_rgb_rejects_bad_input() {
        // TODO: too short, invalid hex digits, non-ASCII input
        todo!("write assertions")
    }

    #[test]
    fn stack_is_lifo_and_bounded() {
        // TODO: push until full, check push returns Err(item), then peek/len/pop in LIFO order
        let _ = BoundedStack::<i32>::new;
        todo!("write assertions")
    }

    // TODO: add #[should_panic(expected = "...")] so this test passes when `new(0)` panics.
    #[test]
    fn stack_with_zero_capacity_panics() {
        todo!("call BoundedStack::<u8>::new(0)")
    }
}
