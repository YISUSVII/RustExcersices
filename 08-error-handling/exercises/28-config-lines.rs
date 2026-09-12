use std::collections::HashMap;

fn parse_config(text: &str) -> Result<HashMap<String, String>, String> {
    // TODO: parse key=value lines; skip blanks and # comments
    let _ = text;
    unimplemented!("Implement parse_config")
}

fn main() {
    let text = "# comment\nname = Ada\nlang=rust\n";
    println!("{:?}", parse_config(text));
    println!("{:?}", parse_config("bad line\n"));
}
