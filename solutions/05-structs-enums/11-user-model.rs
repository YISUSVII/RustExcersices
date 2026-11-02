#[derive(Debug, Clone)]
struct User {
    username: String,
    email: String,
    age: u8,
    active: bool,
}

impl User {
    fn new(username: &str, email: &str, age: u8) -> User {
        User {
            username: username.to_string(),
            email: email.to_string(),
            age,
            active: true,
        }
    }

    fn display_name(&self) -> String {
        format!("{} <{}>", self.username, self.email)
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut user = User::new("alice", "alice@example.com", 30);
    println!("{}", user.display_name());
    println!("adult: {}", user.is_adult());
    user.deactivate();
    println!("active: {}", user.active);

    // Bonus: struct update syntax.
    let work = User {
        email: String::from("alice@work.example"),
        ..user.clone()
    };
    println!("{work:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_user_is_active() {
        let u = User::new("bob", "bob@example.com", 17);
        assert!(u.active);
        assert!(!u.is_adult());
        assert_eq!(u.display_name(), "bob <bob@example.com>");
    }

    #[test]
    fn deactivate_works() {
        let mut u = User::new("bob", "bob@example.com", 40);
        u.deactivate();
        assert!(!u.active);
    }
}
