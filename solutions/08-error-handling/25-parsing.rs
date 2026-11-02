use std::num::ParseIntError;

fn parse_numbers(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split(',').map(|s| s.trim().parse::<i64>()).collect()
}

fn sum_numbers(input: &str) -> Result<i64, ParseIntError> {
    let numbers = parse_numbers(input)?;
    Ok(numbers.iter().sum())
}

fn parse_duration(input: &str) -> Result<u64, String> {
    let input = input.trim();
    if input.is_empty() {
        return Err("empty duration".to_string());
    }

    let mut total = 0;
    let mut digits = String::new();
    let mut seen_units = String::new();

    for c in input.chars() {
        if c.is_ascii_digit() {
            digits.push(c);
            continue;
        }
        let multiplier = match c {
            'h' => 3600,
            'm' => 60,
            's' => 1,
            other => return Err(format!("unknown unit '{other}'")),
        };
        if seen_units.contains(c) {
            return Err(format!("repeated unit '{c}'"));
        }
        seen_units.push(c);
        let value: u64 = digits
            .parse()
            .map_err(|_| format!("missing number before '{c}'"))?;
        total = value
            .checked_mul(multiplier)
            .and_then(|secs| secs.checked_add(total))
            .ok_or_else(|| "duration too large".to_string())?;
        digits.clear();
    }

    if !digits.is_empty() {
        return Err(format!("missing unit after {digits}"));
    }
    Ok(total)
}

fn parse_hex_color(input: &str) -> Option<(u8, u8, u8)> {
    let hex = input.strip_prefix('#')?;
    if hex.len() != 6 || !hex.is_ascii() {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((r, g, b))
}

fn main() {
    println!(
        "parse_numbers(\"1, 2, 3\") = {:?}",
        parse_numbers("1, 2, 3")
    );
    println!("sum_numbers(\"10,20,x\") = {:?}", sum_numbers("10,20,x"));
    for d in ["1h30m15s", "10x", "15"] {
        println!("parse_duration({d:?}) = {:?}", parse_duration(d));
    }
    for c in ["#FF8000", "FF8000"] {
        println!("parse_hex_color({c:?}) = {:?}", parse_hex_color(c));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        assert_eq!(parse_numbers(" 4 ,-5,6"), Ok(vec![4, -5, 6]));
        assert!(parse_numbers("1,,2").is_err());
        assert_eq!(sum_numbers("1,2,3"), Ok(6));
    }

    #[test]
    fn durations_ok() {
        assert_eq!(parse_duration("45s"), Ok(45));
        assert_eq!(parse_duration("2m"), Ok(120));
        assert_eq!(parse_duration("1h30m15s"), Ok(5415));
    }

    #[test]
    fn durations_err() {
        assert!(parse_duration("").is_err());
        assert!(parse_duration("h").is_err());
        assert!(parse_duration("5d").is_err());
        assert!(parse_duration("10").is_err());
        assert!(parse_duration("1h2h").is_err());
        assert!(parse_duration("18446744073709551615h").is_err());
    }

    #[test]
    fn hex_colors() {
        assert_eq!(parse_hex_color("#00ff7F"), Some((0, 255, 127)));
        assert_eq!(parse_hex_color("#GG0000"), None);
        assert_eq!(parse_hex_color("#FFF"), None);
        assert_eq!(parse_hex_color("#ÿÿÿ"), None);
    }
}
