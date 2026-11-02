use std::num::ParseIntError;

fn parse_numbers(input: &str) -> Result<Vec<i64>, ParseIntError> {
    // TODO: implement
    let _ = input;
    todo!("split(','), trim, parse, and collect into Result<Vec<i64>, _>")
}

fn sum_numbers(input: &str) -> Result<i64, ParseIntError> {
    // TODO: implement
    let _ = input;
    todo!("Call parse_numbers with ? and sum")
}

fn parse_duration(input: &str) -> Result<u64, String> {
    // TODO: implement
    let _ = input;
    todo!("Accumulate digits, multiply by 3600/60/1 on h/m/s, error on anything else")
}

fn parse_hex_color(input: &str) -> Option<(u8, u8, u8)> {
    // TODO: implement
    let _ = input;
    todo!("strip_prefix('#')?, check length, u8::from_str_radix(.., 16).ok()?")
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
    }

    #[test]
    fn hex_colors() {
        assert_eq!(parse_hex_color("#00ff7F"), Some((0, 255, 127)));
        assert_eq!(parse_hex_color("#GG0000"), None);
        assert_eq!(parse_hex_color("#FFF"), None);
        assert_eq!(parse_hex_color("#ÿÿÿ"), None);
    }
}
