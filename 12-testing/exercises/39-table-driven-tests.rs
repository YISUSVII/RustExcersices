// This file contains working code with TWO hidden bugs (marked BUG #1/#2 so you
// can find them later). Write the table-driven tests first, watch them fail,
// then fix the bugs.
//
//   rustc --edition 2021 --test 39-table-driven-tests.rs -o table_tests && ./table_tests

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
        // BUG #1: plain `+` panics on overflow in debug builds.
        Op::Add => Ok(a + b),
        Op::Sub => a.checked_sub(b).ok_or(CalcError::Overflow),
        Op::Mul => a.checked_mul(b).ok_or(CalcError::Overflow),
        // BUG #2: plain `/` panics on division by zero.
        Op::Div => Ok(a / b),
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

    /// TODO: shared assertion helper.
    /// Call `evaluate(input)` and `assert_eq!` against `expected`, with a message
    /// that includes the case name, the input, the actual and the expected value.
    /// Add `#[track_caller]` so failures point at the calling line.
    fn check_eval(name: &str, input: &str, expected: Result<i64, CalcError>) {
        let _ = (name, input, expected);
        todo!("assert_eq!(evaluate(input), expected, \"case '...': ...\")")
    }

    #[test]
    fn evaluates_valid_expressions() {
        // TODO: build a table of (name, input, expected_value) tuples covering
        // +, -, * and x, /, negative operands and extra whitespace, then loop:
        //   for (name, input, expected) in cases { check_eval(name, input, Ok(expected)); }
        let cases: [(&str, &str, i64); 0] = [];
        for (name, input, expected) in cases {
            check_eval(name, input, Ok(expected));
        }
        todo!("fill in the table above, then delete this line")
    }

    #[test]
    fn reports_each_error_kind() {
        // TODO: one row per CalcError variant (and more than one row where useful):
        // empty/blank input, bad left/right number, unknown operator,
        // too few/too many tokens, division by zero, add overflow, div overflow.
        let cases: [(&str, &str, CalcError); 0] = [];
        for (name, input, expected) in cases {
            check_eval(name, input, Err(expected));
        }
        todo!("fill in the table above, then delete this line")
    }

    #[test]
    fn error_messages_are_human_readable() {
        // TODO: table of (CalcError, expected Display string)
        todo!("assert_eq!(err.to_string(), expected, \"Display for ...\")")
    }

    #[test]
    fn password_rules_table() {
        // TODO: table of (password, expected issues) e.g. ("", &["too short", ...])
        let _ = password_issues;
        todo!("loop over the table and compare password_issues(pw) with expected")
    }
}
