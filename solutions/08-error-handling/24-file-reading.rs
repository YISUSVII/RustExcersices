use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

fn write_sample(path: &Path, contents: &str) -> io::Result<()> {
    fs::write(path, contents)
}

fn read_lines(path: &Path) -> io::Result<Vec<String>> {
    let contents = fs::read_to_string(path)?;
    Ok(contents.lines().map(String::from).collect())
}

fn count_non_empty_lines(path: &Path) -> io::Result<usize> {
    let lines = read_lines(path)?;
    Ok(lines.iter().filter(|l| !l.trim().is_empty()).count())
}

fn read_or_default(path: &Path, default: &str) -> io::Result<String> {
    match fs::read_to_string(path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(default.to_string()),
        other => other,
    }
}

// Bonus: stream lines instead of loading the whole file.
fn read_lines_buffered(path: &Path) -> io::Result<Vec<String>> {
    let reader = BufReader::new(fs::File::open(path)?);
    reader.lines().collect()
}

fn temp_file(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("rust-ex24-{name}-{}.txt", std::process::id()))
}

fn run() -> io::Result<()> {
    let path = temp_file("sample");
    write_sample(&path, "first line\n\nthird line\n")?;
    println!("wrote {}", path.display());

    for (i, line) in read_lines(&path)?.iter().enumerate() {
        println!("{}: {line}", i + 1);
    }
    println!("non-empty lines: {}", count_non_empty_lines(&path)?);
    assert_eq!(read_lines_buffered(&path)?, read_lines(&path)?);

    let missing = temp_file("does-not-exist");
    println!(
        "missing file -> {:?}",
        read_or_default(&missing, "(no config found)")?
    );
    match read_lines(&missing) {
        Ok(lines) => println!("unexpected: {lines:?}"),
        Err(e) => println!("missing file lines -> error: {:?}", e.kind()),
    }

    fs::remove_file(&path)?;
    println!("cleaned up");
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("I/O error: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_then_read() {
        let path = temp_file("test-write-read");
        write_sample(&path, "a\n \nb").unwrap();
        assert_eq!(read_lines(&path).unwrap(), vec!["a", " ", "b"]);
        assert_eq!(count_non_empty_lines(&path).unwrap(), 2);
        assert_eq!(read_lines_buffered(&path).unwrap().len(), 3);
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn missing_file_uses_default() {
        let path = temp_file("test-missing");
        assert_eq!(read_or_default(&path, "dflt").unwrap(), "dflt");
    }

    #[test]
    fn missing_file_propagates_not_found() {
        let path = temp_file("test-missing-2");
        let err = read_lines(&path).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn other_errors_are_not_swallowed() {
        // Reading a directory as a file fails with a kind other than NotFound.
        let dir = std::env::temp_dir();
        assert!(read_or_default(&dir, "dflt").is_err());
    }
}
