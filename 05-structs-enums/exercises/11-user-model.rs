#[derive(Debug, Clone)]
struct User {
    username: String,
    email: String,
    age: u8,
    active: bool,
}

impl User {
    fn new(username: &str, email: &str, age: u8) -> User {
        // TODO: implement
        let _ = (username, email, age);
        todo!("Build a User with owned Strings and active: true")
    }

    fn display_name(&self) -> String {
        // TODO: implement
        todo!("Return \"username <email>\" using format!")
    }

    fn is_adult(&self) -> bool {
        // TODO: implement
        todo!("Return true when age >= 18")
    }

    fn deactivate(&mut self) {
        // TODO: implement
        todo!("Set active to false")
    }
}

fn main() {
    let mut user = User::new("alice", "alice@example.com", 30);
    println!("{}", user.display_name());
    println!("adult: {}", user.is_adult());
    user.deactivate();
    println!("active: {}", user.active);
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
