fn validate_username(name: &str) -> Result<(), String> {
    // TODO: enforce non-empty, len <= 16, [A-Za-z0-9_]
    let _ = name;
    unimplemented!("Implement validate_username")
}

fn main() {
    for name in ["ada_lovelace", "", "bad name!", "way_too_long_username"] {
        println!("{name:?} => {:?}", validate_username(name));
    }
}
