fn run(cmd: Option<&str>) -> Result<&'static str, String> {
    match cmd {
        None => Err(String::from("missing command")),
        Some("help") => Ok("usage: help|version"),
        Some("version") => Ok("1.0.0"),
        Some(other) => Err(format!("unknown command: {other}")),
    }
}

fn main() {
    println!("{:?}", run(Some("help")));
    println!("{:?}", run(Some("version")));
    println!("{:?}", run(Some("nope")));
    println!("{:?}", run(None));
}
