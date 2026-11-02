# Exercise 50 — Declarative Macros

- **Title:** Declarative Macros
- **Objective:** Write `macro_rules!` macros for collection literals and variadic `max!`/`min!` using recursion.
- **Difficulty:** Intermediate
- **Concepts practiced:** `macro_rules!`, fragment specifiers (`expr`), repetitions `$(...),*`, optional trailing commas `$(,)?`, recursive macros, macro hygiene

## Instructions

1. `hashmap!` — build a `HashMap`:
   - `hashmap!{}` → empty map
   - `hashmap!{ k1 => v1, k2 => v2, }` (trailing comma optional)
2. `hashset![a, b, c]` — build a `HashSet` (empty allowed).
3. `max!(...)` and `min!(...)` — accept one or more expressions:
   - base case `max!(x)` → `x`
   - recursive case `max!(x, rest...)` → compare `x` with `max!(rest...)`
4. `square!(expr)` — must evaluate `expr` **exactly once**.

## Expected behavior

- `hashmap!{ "Ana" => 31, "Ben" => 25 }["Ben"]` → `25`
- `hashset![1, 2, 2, 3].len()` → `3`
- `max!(3, 9, 2, 7)` → `9`, `min!(3.5, -1.0, 2.0)` → `-1`, `max!("b", "c", "a")` → `"c"`
- `square!(next())` calls `next()` only once.

## Optional hints

- Use double braces `=> {{ ... }}` so the expansion is a block expression that can declare local variables.
- `$($key:expr => $value:expr),+ $(,)?` matches one or more pairs plus an optional trailing comma.
- A naive `$x * $x` pastes the expression twice — side effects run twice. Bind it with `let v = $x;` first.
- Variables declared inside a macro are hygienic: they cannot clash with the caller's names.

## Bonus challenge

Write `btreemap!` that also supports a capacity-free nested form like `btreemap!{ "a" => btreemap!{ 1 => 2 } }`, and a `count!` macro that counts its arguments at compile time (`count!(a, b, c)` → `3usize`).
