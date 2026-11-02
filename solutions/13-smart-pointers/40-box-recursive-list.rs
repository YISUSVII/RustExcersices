use std::fmt;

#[derive(Debug, PartialEq)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn from_slice(values: &[i32]) -> List {
        values
            .iter()
            .rev()
            .fold(Nil, |acc, &v| Cons(v, Box::new(acc)))
    }

    fn len(&self) -> usize {
        match self {
            Cons(_, rest) => 1 + rest.len(),
            Nil => 0,
        }
    }

    fn sum(&self) -> i32 {
        match self {
            Cons(v, rest) => v + rest.sum(),
            Nil => 0,
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cons(v, rest) => write!(f, "({v} {rest})"),
            Nil => write!(f, "Nil"),
        }
    }
}

#[derive(Debug)]
enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn eval(&self) -> Result<f64, String> {
        match self {
            Expr::Num(n) => Ok(*n),
            Expr::Add(a, b) => Ok(a.eval()? + b.eval()?),
            Expr::Sub(a, b) => Ok(a.eval()? - b.eval()?),
            Expr::Mul(a, b) => Ok(a.eval()? * b.eval()?),
            Expr::Div(a, b) => {
                let divisor = b.eval()?;
                if divisor == 0.0 {
                    Err("division by zero".to_string())
                } else {
                    Ok(a.eval()? / divisor)
                }
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{n}"),
            Expr::Add(a, b) => write!(f, "({a} + {b})"),
            Expr::Sub(a, b) => write!(f, "({a} - {b})"),
            Expr::Mul(a, b) => write!(f, "({a} * {b})"),
            Expr::Div(a, b) => write!(f, "({a} / {b})"),
        }
    }
}

fn num(n: f64) -> Box<Expr> {
    Box::new(Expr::Num(n))
}

fn main() {
    let list = List::from_slice(&[1, 2, 3]);
    println!("list = {list}");
    println!("len = {}, sum = {}", list.len(), list.sum());

    // (2 + 3) * (10 / 4)
    let expr = Expr::Mul(
        Box::new(Expr::Add(num(2.0), num(3.0))),
        Box::new(Expr::Div(num(10.0), num(4.0))),
    );
    println!("{expr} = {:?}", expr.eval());

    let bad = Expr::Div(num(1.0), Box::new(Expr::Sub(num(2.0), num(2.0))));
    println!("{bad} = {:?}", bad.eval());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_len_and_sum() {
        let list = List::from_slice(&[4, 5, 6, 7]);
        assert_eq!(list.len(), 4);
        assert_eq!(list.sum(), 22);
        assert_eq!(List::from_slice(&[]), Nil);
    }

    #[test]
    fn list_display() {
        assert_eq!(List::from_slice(&[1, 2]).to_string(), "(1 (2 Nil))");
    }

    #[test]
    fn expr_eval() {
        let e = Expr::Sub(num(10.0), Box::new(Expr::Mul(num(2.0), num(3.0))));
        assert_eq!(e.eval(), Ok(4.0));
        assert_eq!(e.to_string(), "(10 - (2 * 3))");
    }

    #[test]
    fn expr_division_by_zero() {
        let e = Expr::Div(num(1.0), num(0.0));
        assert!(e.eval().is_err());
    }
}
