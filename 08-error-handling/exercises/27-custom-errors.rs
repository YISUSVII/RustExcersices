use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
struct Config {
    host: String,
    port: u16,
    workers: u8,
}

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse { key: String, source: ParseIntError },
    MissingKey(&'static str),
    Invalid(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: implement
        let _ = f;
        todo!("Short message per variant, e.g. \"I/O error\", \"missing key 'workers'\"")
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // TODO: implement
        todo!("Return Some(inner) for Io and Parse, None otherwise")
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        // TODO: implement
        let _ = e;
        todo!("Wrap the io::Error in AppError::Io")
    }
}

fn parse_config(text: &str) -> Result<Config, AppError> {
    // TODO: implement
    let _ = text;
    todo!("Parse key=value lines, then validate host, port (u16 != 0) and workers (1..=64)")
}

fn load_config(path: &Path) -> Result<Config, AppError> {
    // TODO: implement
    let _ = path;
    todo!("fs::read_to_string(path)? then parse_config")
}

fn print_error_chain(err: &dyn Error) {
    println!("error: {err}");
    let mut source = err.source();
    while let Some(cause) = source {
        println!("  caused by: {cause}");
        source = cause.source();
    }
}

fn temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("rust-ex27-{name}-{}.conf", std::process::id()))
}

fn run() -> Result<(), Box<dyn Error>> {
    let samples = [
        (
            "good",
            "# demo\nhost = localhost\nport = 8080\nworkers = 4\n",
        ),
        ("bad-port", "host=localhost\nport=80a\nworkers=4\n"),
        ("no-workers", "host=localhost\nport=8080\n"),
        ("too-many", "host=localhost\nport=8080\nworkers=99\n"),
    ];

    let mut paths = Vec::new();
    for (name, contents) in samples {
        let path = temp_path(name);
        fs::write(&path, contents)?;
        paths.push(path);
    }

    let config = load_config(&paths[0])?;
    println!("loaded: {config:?}");

    if let Err(e) = load_config(&temp_path("missing")) {
        print_error_chain(&e);
    }
    for path in &paths[1..] {
        if let Err(e) = load_config(path) {
            print_error_chain(&e);
        }
    }

    for path in &paths {
        fs::remove_file(path)?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        print_error_chain(e.as_ref());
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_config() {
        let cfg = parse_config("host=a\n\n# c\nport=1\nworkers=64").unwrap();
        assert_eq!(
            cfg,
            Config {
                host: "a".into(),
                port: 1,
                workers: 64
            }
        );
    }

    #[test]
    fn parse_error_keeps_source() {
        let err = parse_config("host=a\nport=99999\nworkers=1").unwrap_err();
        assert!(matches!(&err, AppError::Parse { key, .. } if key == "port"));
        assert!(err.source().is_some());
        assert_eq!(err.to_string(), "invalid value for 'port'");
    }

    #[test]
    fn missing_and_invalid() {
        assert!(matches!(
            parse_config("port=1\nworkers=1"),
            Err(AppError::MissingKey("host"))
        ));
        assert!(matches!(
            parse_config("host=a\nport=0\nworkers=1"),
            Err(AppError::Invalid(_))
        ));
        assert!(matches!(
            parse_config("host=a\nport=1\nworkers=0"),
            Err(AppError::Invalid(_))
        ));
        assert!(matches!(
            parse_config("nonsense"),
            Err(AppError::Invalid(_))
        ));
    }

    #[test]
    fn io_error_converts_with_question_mark() {
        let err = load_config(&temp_path("test-missing")).unwrap_err();
        match &err {
            AppError::Io(e) => assert_eq!(e.kind(), io::ErrorKind::NotFound),
            other => panic!("expected Io error, got {other:?}"),
        }
        assert!(err.source().is_some());
    }

    #[test]
    fn load_valid_file() {
        let path = temp_path("test-good");
        fs::write(&path, "host=h\nport=2\nworkers=3").unwrap();
        let cfg = load_config(&path);
        fs::remove_file(&path).unwrap();
        assert_eq!(cfg.unwrap().workers, 3);
    }
}
