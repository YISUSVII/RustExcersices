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
        match self {
            AppError::Io(_) => write!(f, "I/O error"),
            AppError::Parse { key, .. } => write!(f, "invalid value for '{key}'"),
            AppError::MissingKey(key) => write!(f, "missing key '{key}'"),
            AppError::Invalid(msg) => write!(f, "invalid config: {msg}"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            AppError::Parse { source, .. } => Some(source),
            AppError::MissingKey(_) | AppError::Invalid(_) => None,
        }
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

fn parse_field<T: std::str::FromStr<Err = ParseIntError>>(
    key: &'static str,
    value: Option<&str>,
) -> Result<T, AppError> {
    let raw = value.ok_or(AppError::MissingKey(key))?;
    raw.parse().map_err(|source| AppError::Parse {
        key: key.to_string(),
        source,
    })
}

fn parse_config(text: &str) -> Result<Config, AppError> {
    let mut host = None;
    let mut port = None;
    let mut workers = None;

    for line in text.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| AppError::Invalid(format!("expected key=value, got '{line}'")))?;
        let value = value.trim();
        match key.trim() {
            "host" => host = Some(value),
            "port" => port = Some(value),
            "workers" => workers = Some(value),
            other => return Err(AppError::Invalid(format!("unknown key '{other}'"))),
        }
    }

    let host = host.ok_or(AppError::MissingKey("host"))?;
    if host.is_empty() {
        return Err(AppError::Invalid("host must not be empty".to_string()));
    }
    let port: u16 = parse_field("port", port)?;
    if port == 0 {
        return Err(AppError::Invalid("port must not be 0".to_string()));
    }
    let workers: u8 = parse_field("workers", workers)?;
    if !(1..=64).contains(&workers) {
        return Err(AppError::Invalid(format!(
            "workers must be in 1..=64, got {workers}"
        )));
    }

    Ok(Config {
        host: host.to_string(),
        port,
        workers,
    })
}

fn load_config(path: &Path) -> Result<Config, AppError> {
    let text = fs::read_to_string(path)?;
    parse_config(&text)
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
