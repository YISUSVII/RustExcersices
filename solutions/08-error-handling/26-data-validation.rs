use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct SignupForm<'a> {
    username: &'a str,
    email: &'a str,
    age: &'a str,
    password: &'a str,
}

#[derive(Debug, PartialEq)]
struct ValidUser {
    username: String,
    email: String,
    age: u8,
}

#[derive(Debug, PartialEq)]
enum ValidationError {
    EmptyUsername,
    UsernameTooLong { max: usize },
    InvalidUsernameChar(char),
    InvalidEmail(String),
    AgeNotANumber(String),
    AgeOutOfRange(u32),
    WeakPassword(&'static str),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::EmptyUsername => write!(f, "username must not be empty"),
            ValidationError::UsernameTooLong { max } => {
                write!(f, "username must be at most {max} characters")
            }
            ValidationError::InvalidUsernameChar(c) => {
                write!(f, "username contains invalid character '{c}'")
            }
            ValidationError::InvalidEmail(email) => write!(f, "email '{email}' is invalid"),
            ValidationError::AgeNotANumber(raw) => write!(f, "age must be a number, got '{raw}'"),
            ValidationError::AgeOutOfRange(age) => write!(f, "age {age} is outside 13..=120"),
            ValidationError::WeakPassword(reason) => write!(f, "password is too weak: {reason}"),
        }
    }
}

impl Error for ValidationError {}

const MAX_USERNAME: usize = 16;

fn validate_username(raw: &str) -> Result<String, ValidationError> {
    let name = raw.trim();
    if name.is_empty() {
        return Err(ValidationError::EmptyUsername);
    }
    if name.chars().count() > MAX_USERNAME {
        return Err(ValidationError::UsernameTooLong { max: MAX_USERNAME });
    }
    if let Some(bad) = name.chars().find(|c| !(c.is_alphanumeric() || *c == '_')) {
        return Err(ValidationError::InvalidUsernameChar(bad));
    }
    Ok(name.to_string())
}

fn validate_email(raw: &str) -> Result<String, ValidationError> {
    let email = raw.trim();
    let invalid = || ValidationError::InvalidEmail(email.to_string());
    let (local, domain) = email.split_once('@').ok_or_else(invalid)?;
    let domain_ok = domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.');
    if local.is_empty() || domain.contains('@') || !domain_ok {
        return Err(invalid());
    }
    Ok(email.to_string())
}

fn validate_age(raw: &str) -> Result<u8, ValidationError> {
    let age: u32 = raw
        .trim()
        .parse()
        .map_err(|_| ValidationError::AgeNotANumber(raw.to_string()))?;
    match age {
        13..=120 => Ok(age as u8),
        _ => Err(ValidationError::AgeOutOfRange(age)),
    }
}

fn validate_password(raw: &str) -> Result<(), ValidationError> {
    if raw.chars().count() < 8 {
        Err(ValidationError::WeakPassword("needs at least 8 characters"))
    } else if !raw.chars().any(|c| c.is_ascii_digit()) {
        Err(ValidationError::WeakPassword("needs a digit"))
    } else if !raw.chars().any(char::is_uppercase) {
        Err(ValidationError::WeakPassword("needs an uppercase letter"))
    } else {
        Ok(())
    }
}

fn validate(form: &SignupForm) -> Result<ValidUser, ValidationError> {
    let username = validate_username(form.username)?;
    let email = validate_email(form.email)?;
    let age = validate_age(form.age)?;
    validate_password(form.password)?;
    Ok(ValidUser {
        username,
        email,
        age,
    })
}

fn validate_all(form: &SignupForm) -> Result<ValidUser, Vec<ValidationError>> {
    let username = validate_username(form.username);
    let email = validate_email(form.email);
    let age = validate_age(form.age);
    let password = validate_password(form.password);

    match (username, email, age, password) {
        (Ok(username), Ok(email), Ok(age), Ok(())) => Ok(ValidUser {
            username,
            email,
            age,
        }),
        (username, email, age, password) => {
            let errors = [username.err(), email.err(), age.err(), password.err()]
                .into_iter()
                .flatten()
                .collect();
            Err(errors)
        }
    }
}

// Bonus: ValidationError works as a boxed trait object thanks to `impl Error`.
fn validate_boxed(form: &SignupForm) -> Result<ValidUser, Box<dyn Error>> {
    Ok(validate(form)?)
}

fn main() {
    let good = SignupForm {
        username: "ferris",
        email: "ferris@rust-lang.org",
        age: "30",
        password: "Crab1234",
    };
    let bad_email = SignupForm {
        email: "nope",
        ..good
    };
    let all_bad = SignupForm {
        username: "ferris!",
        email: "nope",
        age: "ten",
        password: "short",
    };

    match validate(&good) {
        Ok(user) => println!("valid: {user:?}"),
        Err(e) => println!("unexpected error: {e}"),
    }
    if let Err(e) = validate_boxed(&bad_email) {
        println!("first error: {e}");
    }
    if let Err(errors) = validate_all(&all_bad) {
        println!("all errors:");
        for e in errors {
            println!("  - {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn form() -> SignupForm<'static> {
        SignupForm {
            username: "ferris",
            email: "f@rust.org",
            age: "30",
            password: "Crab1234",
        }
    }

    #[test]
    fn valid_form_passes() {
        let user = validate(&form()).unwrap();
        assert_eq!(user.age, 30);
        assert_eq!(validate_all(&form()), Ok(user));
    }

    #[test]
    fn field_validators() {
        assert_eq!(validate_username("  "), Err(ValidationError::EmptyUsername));
        assert_eq!(
            validate_username("a_very_long_username"),
            Err(ValidationError::UsernameTooLong { max: 16 })
        );
        assert!(validate_email("a@b").is_err());
        assert!(validate_email("@b.com").is_err());
        assert!(validate_email("a@b.com").is_ok());
        assert_eq!(
            validate_age("500"),
            Err(ValidationError::AgeOutOfRange(500))
        );
        assert_eq!(
            validate_age("x"),
            Err(ValidationError::AgeNotANumber("x".into()))
        );
        assert_eq!(
            validate_password("crab1234"),
            Err(ValidationError::WeakPassword("needs an uppercase letter"))
        );
    }

    #[test]
    fn validate_stops_at_first_error() {
        let f = SignupForm {
            username: "",
            age: "1",
            ..form()
        };
        assert_eq!(validate(&f), Err(ValidationError::EmptyUsername));
    }

    #[test]
    fn validate_all_collects_every_error() {
        let f = SignupForm {
            username: "",
            email: "x",
            age: "1",
            password: "p",
        };
        assert_eq!(validate_all(&f).unwrap_err().len(), 4);
    }

    #[test]
    fn display_messages() {
        assert_eq!(
            ValidationError::InvalidEmail("x".into()).to_string(),
            "email 'x' is invalid"
        );
    }
}
