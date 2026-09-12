use std::collections::HashMap;

fn parse_config(text: &str) -> Result<HashMap<String, String>, String> {
    let mut map = HashMap::new();
    for (idx, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("line {}: missing '='", idx + 1))?;
        let key = key.trim().to_string();
        let value = value.trim().to_string();
        if key.is_empty() {
            return Err(format!("line {}: empty key", idx + 1));
        }
        if map.insert(key.clone(), value).is_some() {
            return Err(format!("duplicate key: {key}"));
        }
    }
    Ok(map)
}

fn main() {
    let text = "# comment\nname = Ada\nlang=rust\n";
    println!("{:?}", parse_config(text));
    println!("{:?}", parse_config("bad line\n"));
}
