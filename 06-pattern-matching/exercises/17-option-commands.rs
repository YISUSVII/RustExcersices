fn run(cmd: Option<&str>) -> Result<&'static str, String> {
    // TODO: match on cmd as described in the exercise markdown
    let _ = cmd;
    unimplemented!("Implement run")
}

fn main() {
    println!("{:?}", run(Some("help")));
    println!("{:?}", run(Some("version")));
    println!("{:?}", run(Some("nope")));
    println!("{:?}", run(None));
}
