use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
enum CalcError {
    Empty,
    BadNumber(String),
    UnknownOperator(String),
    WrongTokenCount(usize),
    DivisionByZero,
    Overflow,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::Empty => write!(f, "empty expression"),
            CalcError::BadNumber(s) => write!(f, "bad number: '{s}'"),
            CalcError::UnknownOperator(s) => write!(f, "unknown operator: '{s}'"),
            CalcError::WrongTokenCount(n) => write!(f, "expected 3 tokens, got {n}"),
            CalcError::DivisionByZero => write!(f, "division by zero"),
            CalcError::Overflow => write!(f, "arithmetic overflow"),
        }
    }
}

impl std::error::Error for CalcError {}

fn parse_op(token: &str) -> Result<Op, CalcError> {
    match token {
        "+" => Ok(Op::Add),
        "-" => Ok(Op::Sub),
        "*" | "x" => Ok(Op::Mul),
        "/" => Ok(Op::Div),
        other => Err(CalcError::UnknownOperator(other.to_string())),
    }
}

fn parse_number(token: &str) -> Result<i64, CalcError> {
    token
        .parse()
        .map_err(|_| CalcError::BadNumber(token.to_string()))
}

fn apply(op: Op, a: i64, b: i64) -> Result<i64, CalcError> {
    match op {
        Op::Add => a.checked_add(b).ok_or(CalcError::Overflow),
        Op::Sub => a.checked_sub(b).ok_or(CalcError::Overflow),
        Op::Mul => a.checked_mul(b).ok_or(CalcError::Overflow),
        Op::Div if b == 0 => Err(CalcError::DivisionByZero),
        Op::Div => a.checked_div(b).ok_or(CalcError::Overflow),
    }
}

/// Evaluates expressions of the form "<int> <op> <int>", e.g. "12 * -3".
fn evaluate(expr: &str) -> Result<i64, CalcError> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    match tokens.as_slice() {
        [] => Err(CalcError::Empty),
        [a, op, b] => apply(parse_op(op)?, parse_number(a)?, parse_number(b)?),
        other => Err(CalcError::WrongTokenCount(other.len())),
    }
}

/// Classifies a password; returns every rule that failed (empty = strong).
fn password_issues(pw: &str) -> Vec<&'static str> {
    let rules: [(&'static str, fn(&str) -> bool); 4] = [
        ("too short", |p| p.chars().count() >= 8),
        ("no digit", |p| p.chars().any(|c| c.is_ascii_digit())),
        ("no uppercase", |p| p.chars().any(char::is_uppercase)),
        ("no lowercase", |p| p.chars().any(char::is_lowercase)),
    ];
    rules
        .iter()
        .filter(|(_, ok)| !ok(pw))
        .map(|(name, _)| *name)
        .collect()
}

fn main() {
    for expr in [
        "2 + 3", "12 * -3", "7 / 2", "1 / 0", "4 % 2", "1 +", "", "a + 1",
    ] {
        match evaluate(expr) {
            Ok(v) => println!("{expr:>10} = {v}"),
            Err(e) => println!("{expr:>10} -> error: {e}"),
        }
    }
    for pw in ["short", "longenough", "Str0ngPassword"] {
        println!("{pw:>15}: {:?}", password_issues(pw));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Shared assertion helper: reports the failing case's name and input.
    #[track_caller]
    fn check_eval(name: &str, input: &str, expected: Result<i64, CalcError>) {
        let actual = evaluate(input);
        assert_eq!(
            actual, expected,
            "case '{name}': evaluate({input:?}) returned {actual:?}, expected {expected:?}"
        );
    }

    #[test]
    fn evaluates_valid_expressions() {
        let cases = [
            ("addition", "2 + 3", 5),
            ("subtraction to negative", "2 - 5", -3),
            ("multiplication with x", "4 x 5", 20),
            ("negative operand", "12 * -3", -36),
            ("integer division truncates", "7 / 2", 3),
            ("extra whitespace", "  10   -   1 ", 9),
        ];
        for (name, input, expected) in cases {
            check_eval(name, input, Ok(expected));
        }
    }

    #[test]
    fn reports_each_error_kind() {
        let cases = [
            ("empty", "", CalcError::Empty),
            ("blank", "   ", CalcError::Empty),
            ("bad left", "a + 1", CalcError::BadNumber("a".into())),
            ("bad right", "1 + 2.5", CalcError::BadNumber("2.5".into())),
            (
                "unknown op",
                "4 % 2",
                CalcError::UnknownOperator("%".into()),
            ),
            ("too few tokens", "1 +", CalcError::WrongTokenCount(2)),
            (
                "too many tokens",
                "1 + 2 + 3",
                CalcError::WrongTokenCount(5),
            ),
            ("div by zero", "1 / 0", CalcError::DivisionByZero),
            (
                "add overflow",
                "9223372036854775807 + 1",
                CalcError::Overflow,
            ),
            (
                "div overflow",
                "-9223372036854775808 / -1",
                CalcError::Overflow,
            ),
        ];
        for (name, input, expected) in cases {
            check_eval(name, input, Err(expected));
        }
    }

    #[test]
    fn error_messages_are_human_readable() {
        let cases = [
            (CalcError::Empty, "empty expression"),
            (CalcError::BadNumber("z".into()), "bad number: 'z'"),
            (CalcError::WrongTokenCount(1), "expected 3 tokens, got 1"),
            (CalcError::DivisionByZero, "division by zero"),
        ];
        for (err, expected) in cases {
            assert_eq!(err.to_string(), expected, "Display for {err:?}");
        }
    }

    #[test]
    fn password_rules_table() {
        let cases: &[(&str, &[&str])] = &[
            (
                "",
                &["too short", "no digit", "no uppercase", "no lowercase"],
            ),
            ("short", &["too short", "no digit", "no uppercase"]),
            ("longenough", &["no digit", "no uppercase"]),
            ("ALLCAPS123", &["no lowercase"]),
            ("Str0ngPassword", &[]),
        ];
        for &(pw, expected) in cases {
            assert_eq!(
                password_issues(pw),
                expected,
                "password_issues({pw:?}) mismatch"
            );
        }
    }
}
