pub fn parse_pair(raw: &str) -> Result<(i32, i32), String> {
    // TODO: parse "LxW" like "2x3"
    let _ = raw;
    unimplemented!("Implement parse_pair")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic() {
        assert_eq!(parse_pair("2x3").unwrap(), (2, 3));
    }
}
