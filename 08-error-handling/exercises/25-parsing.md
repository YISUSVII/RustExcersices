# Exercise 25 — Parsing

- **Title:** Parsing with `Result`
- **Objective:** Turn untrusted strings into typed values, propagating or converting errors instead of panicking.
- **Difficulty:** Intermediate
- **Concepts practiced:** `str::parse`, `ParseIntError`, collecting into `Result<Vec<_>, _>`, `?`, `map_err`, `ok()?` on `Option`

## Instructions

1. Implement `parse_numbers(input: &str) -> Result<Vec<i64>, ParseIntError>` for comma-separated values like `"1, 2, 3"` (trim each item). The first bad item fails the whole parse.
2. Implement `sum_numbers(input: &str) -> Result<i64, ParseIntError>` using `parse_numbers` and `?`.
3. Implement `parse_duration(input: &str) -> Result<u64, String>` that converts strings like `"1h30m15s"`, `"45s"` or `"2m"` into seconds:
   - units are `h`, `m`, `s`;
   - errors (as `String`) for empty input, a missing number before a unit, an unknown unit, or trailing digits without a unit.
4. Implement `parse_hex_color(input: &str) -> Option<(u8, u8, u8)>` for `"#RRGGBB"` using `u8::from_str_radix(..).ok()?`.

## Expected behavior

```text
parse_numbers("1, 2, 3") = Ok([1, 2, 3])
sum_numbers("10,20,x") = Err(ParseIntError { kind: InvalidDigit })
parse_duration("1h30m15s") = Ok(5415)
parse_duration("10x") = Err("unknown unit 'x'")
parse_duration("15") = Err("missing unit after 15")
parse_hex_color("#FF8000") = Some((255, 128, 0))
parse_hex_color("FF8000") = None
```

## Optional hints

- `input.split(',').map(|s| s.trim().parse::<i64>()).collect::<Result<Vec<_>, _>>()`
- For durations, accumulate digits in a `String` and flush them when you meet a unit letter.
- `str::strip_prefix('#')` returns an `Option`.

## Bonus challenge

Make `parse_duration` reject repeated units such as `"1h2h"`.
