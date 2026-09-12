pub fn parse_pair(raw: &str) -> Result<(i32, i32), String> {
    let (left, right) = raw
        .split_once('x')
        .ok_or_else(|| String::from("missing x"))?;
    let l: i32 = left
        .trim()
        .parse()
        .map_err(|e| format!("left: {e}"))?;
    let r: i32 = right
        .trim()
        .parse()
        .map_err(|e| format!("right: {e}"))?;
    Ok((l, r))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic() {
        assert_eq!(parse_pair("2x3").unwrap(), (2, 3));
    }
}
