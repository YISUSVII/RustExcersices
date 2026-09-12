fn validate_username(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err(String::from("username must not be empty"));
    }
    if name.len() > 16 {
        return Err(String::from("username must be at most 16 characters"));
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err(String::from(
            "username must be ASCII alphanumeric or underscore",
        ));
    }
    Ok(())
}

fn main() {
    for name in ["ada_lovelace", "", "bad name!", "way_too_long_username"] {
        println!("{name:?} => {:?}", validate_username(name));
    }
}
