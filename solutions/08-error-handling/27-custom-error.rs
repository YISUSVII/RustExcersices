#[derive(Debug, PartialEq, Eq)]
enum AppError {
    Empty,
    NotANumber,
    TooBig,
}

fn parse_score(raw: &str) -> Result<u8, AppError> {
    if raw.trim().is_empty() {
        return Err(AppError::Empty);
    }
    let value: u32 = raw.trim().parse().map_err(|_| AppError::NotANumber)?;
    if value > 100 {
        return Err(AppError::TooBig);
    }
    Ok(value as u8)
}

fn main() {
    for raw in ["85", "", "x", "250"] {
        println!("{raw:?} => {:?}", parse_score(raw));
    }
}
