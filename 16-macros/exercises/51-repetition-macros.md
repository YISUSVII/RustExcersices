# Exercise 51 — Repetition Macros

- **Title:** Repetition Macros
- **Objective:** Use macro repetitions to generate whole items: an enum with `Display`/`FromStr`, a config struct with defaults and getters, and a family of test functions.
- **Difficulty:** Advanced
- **Concepts practiced:** `ident`, `ty`, `literal`, `path`, `expr` fragments, item-generating macros, nested repetitions, `stringify!`, code generation for tests

## Instructions

1. `string_enum! { Name { Variant => "text", ... } }` must generate:
   - `enum Name { Variant, ... }` deriving `Debug, Clone, Copy, PartialEq, Eq`
   - `const ALL: &'static [Name]` listing every variant
   - `impl fmt::Display` printing each variant's text
   - `impl FromStr` (`Err = String`) parsing the text back (unknown → `Err`)
2. `config_struct! { Name { field: Type = default_expr, ... } }` must generate:
   - the struct, `impl Default` using the default expressions
   - one getter per field: `fn field(&self) -> &Type`
   - `fn field_names() -> &'static [&'static str]`
3. `test_cases! { func; case_name: (input, expected), ... }` must generate one `#[test] fn case_name()` per case asserting `func(input) == expected`.
4. Use them for `Color { Red, Green, Blue }`, `ServerConfig { host, port, workers }`, and `parse_color` tests.

## Expected behavior

- `Color::ALL.len()` → `3`; `Color::Green.to_string()` → `"green"`; `"pink".parse::<Color>()` → `Err("unknown Color: \"pink\"")`.
- `ServerConfig::default().port()` → `&8080`; `ServerConfig::field_names()` → `["host", "port", "workers"]`.
- `cargo test`/`rustc --test` lists `parses_red`, `parses_blue`, `rejects_unknown`, `is_case_sensitive` as separate tests.

## Optional hints

- A macro can expand to several items at once — just write them one after another in the transcriber.
- Getter names can reuse the field identifiers: `$( fn $field(&self) -> &$ty { &self.$field } )+`.
- `stringify!($field)` turns an identifier into a `&'static str` literal.
- In `FromStr`, literal patterns work in `match`: `$($text => Ok($name::$variant),)+`.

## Bonus challenge

Add per-field validation to `config_struct!` (e.g. `port: u16 = 8080 => |p| *p > 0`) generating a `fn validate(&self) -> Result<(), String>`, and add a `with_<field>` builder method per field (hint: identifier concatenation needs the `paste` crate — or a different design!).
