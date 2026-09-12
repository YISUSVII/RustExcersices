#[derive(Debug, PartialEq, Eq)]
enum AppError {
    Empty,
    NotANumber,
    TooBig,
}

fn parse_score(raw: &str) -> Result<u8, AppError> {
    // TODO: map failure modes to AppError variants
    let _ = raw;
    unimplemented!("Implement parse_score")
}

fn main() {
    for raw in ["85", "", "x", "250"] {
        println!("{raw:?} => {:?}", parse_score(raw));
    }
}
