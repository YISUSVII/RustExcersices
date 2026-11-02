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
        // TODO: implement
        let _ = f;
        todo!("match self and write! a human-readable message for each variant")
    }
}

const MAX_USERNAME: usize = 16;

fn validate_username(raw: &str) -> Result<String, ValidationError> {
    // TODO: implement
    let _ = (raw, MAX_USERNAME);
    todo!("trim, then check empty / too long / invalid characters")
}

fn validate_email(raw: &str) -> Result<String, ValidationError> {
    // TODO: implement
    let _ = raw;
    todo!("split_once('@') and check both parts")
}

fn validate_age(raw: &str) -> Result<u8, ValidationError> {
    // TODO: implement
    let _ = raw;
    todo!("parse to u32 with map_err, then range-check 13..=120")
}

fn validate_password(raw: &str) -> Result<(), ValidationError> {
    // TODO: implement
    let _ = raw;
    todo!("length >= 8, contains a digit, contains an uppercase letter")
}

fn validate(form: &SignupForm) -> Result<ValidUser, ValidationError> {
    // TODO: implement
    let _ = form;
    todo!("Call each validator with ? and build ValidUser")
}

fn validate_all(form: &SignupForm) -> Result<ValidUser, Vec<ValidationError>> {
    // TODO: implement
    let _ = form;
    todo!("Run every validator and collect all errors into a Vec")
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
    if let Err(e) = validate(&bad_email) {
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
