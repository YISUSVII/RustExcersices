# Exercise 40 — Box Recursive List

- **Title:** Box Recursive List
- **Objective:** Use `Box<T>` to build recursive data types: a cons list and an arithmetic expression tree.
- **Difficulty:** Intermediate
- **Concepts practiced:** `Box<T>`, recursive enums, pattern matching, recursion, `Display`

## Instructions

1. Define a cons list:
   ```rust
   enum List { Cons(i32, Box<List>), Nil }
   ```
2. Implement on `List`:
   - `fn from_slice(values: &[i32]) -> List`
   - `fn len(&self) -> usize`
   - `fn sum(&self) -> i32`
   - `impl fmt::Display for List` producing `(1 (2 (3 Nil)))`.
3. Define an expression tree:
   ```rust
   enum Expr { Num(f64), Add(Box<Expr>, Box<Expr>), Sub(..), Mul(..), Div(..) }
   ```
4. Implement `fn eval(&self) -> Result<f64, String>` returning `Err` on division by zero.
5. Implement `impl fmt::Display for Expr` printing fully parenthesized expressions, e.g. `((2 + 3) * (10 / 4))`.

## Expected behavior

- `List::from_slice(&[1, 2, 3])` displays as `(1 (2 (3 Nil)))`, `len() == 3`, `sum() == 6`.
- `((2 + 3) * (10 / 4))` evaluates to `Ok(12.5)`.
- `(1 / (2 - 2))` evaluates to `Err("division by zero")`.

## Optional hints

- Without `Box`, `List` would have infinite size; `Box` gives the recursive field a known size (one pointer).
- `from_slice` is a nice fit for `iter().rev().fold(Nil, ...)`.
- Use the `?` operator inside `eval` to propagate errors from sub-expressions.

## Bonus challenge

Add a `Neg(Box<Expr>)` variant and a `simplify(self) -> Expr` method that folds constant sub-trees (e.g. `Add(Num(1), Num(2))` → `Num(3)`).
