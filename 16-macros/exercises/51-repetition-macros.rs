#![allow(dead_code, unused_imports)]
use std::fmt;
use std::str::FromStr;

/// Generates a fieldless enum with `Display`, `FromStr`, and an `ALL` constant.
macro_rules! string_enum {
    ($name:ident { $($variant:ident => $text:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum $name {
            $($variant),+
        }

        impl $name {
            // TODO: list every variant: &[$($name::$variant),+]
            const ALL: &'static [$name] = &[];
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // TODO: match each variant to its $text
                let _ = f;
                todo!("one match arm per variant")
            }
        }

        impl FromStr for $name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                // TODO: match each $text back to its variant; unknown => Err(..)
                let _ = s;
                todo!("Generate one match arm per variant")
            }
        }
    };
}

/// Generates a struct with a `Default` impl (from the given defaults),
/// a getter per field, and a `field_names()` list.
macro_rules! config_struct {
    ($name:ident { $($field:ident : $ty:ty = $default:expr),+ $(,)? }) => {
        #[derive(Debug, Clone, PartialEq)]
        struct $name {
            $($field: $ty),+
        }

        impl Default for $name {
            fn default() -> Self {
                // TODO: build the struct from the $default expressions
                todo!("build the struct from the defaults")
            }
        }

        impl $name {
            $(
                fn $field(&self) -> &$ty {
                    // TODO: return a reference to the field
                    todo!()
                }
            )+

            fn field_names() -> &'static [&'static str] {
                // TODO: use stringify!($field) for each field
                &[]
            }
        }
    };
}

/// Generates one `#[test]` function per case: `name: (input, expected)`.
#[allow(unused_macros)]
macro_rules! test_cases {
    ($func:path; $($name:ident : ($input:expr, $expected:expr)),+ $(,)?) => {
        // TODO: for each case generate:
        // #[test] fn $name() { assert_eq!($func($input), $expected); }
    };
}

string_enum! {
    Color {
        Red => "red",
        Green => "green",
        Blue => "blue",
    }
}

config_struct! {
    ServerConfig {
        host: String = String::from("localhost"),
        port: u16 = 8080,
        workers: usize = 4,
    }
}

fn parse_color(s: &str) -> Option<Color> {
    s.parse().ok()
}

fn main() {
    for c in Color::ALL {
        println!("{c:?} displays as {c}");
    }
    println!("parse \"green\" -> {:?}", parse_color("green"));
    println!("parse \"pink\"  -> {:?}", "pink".parse::<Color>());

    let cfg = ServerConfig::default();
    println!("fields: {:?}", ServerConfig::field_names());
    println!("host = {}, port = {}, workers = {}", cfg.host(), cfg.port(), cfg.workers());

    let custom = ServerConfig {
        port: 3000,
        ..ServerConfig::default()
    };
    println!("custom = {custom:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_round_trips() {
        for c in Color::ALL {
            assert_eq!(c.to_string().parse::<Color>(), Ok(*c));
        }
        assert_eq!(Color::ALL.len(), 3);
        assert!("purple".parse::<Color>().is_err());
    }

    #[test]
    fn config_defaults_and_getters() {
        let cfg = ServerConfig::default();
        assert_eq!(cfg.host(), "localhost");
        assert_eq!(*cfg.port(), 8080);
        assert_eq!(ServerConfig::field_names(), ["host", "port", "workers"]);
    }

    test_cases! { parse_color;
        parses_red: ("red", Some(Color::Red)),
        parses_blue: ("blue", Some(Color::Blue)),
        rejects_unknown: ("teal", None),
        is_case_sensitive: ("Red", None),
    }
}
