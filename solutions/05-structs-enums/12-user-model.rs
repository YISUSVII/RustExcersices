struct User {
    name: String,
    age: u8,
    active: bool,
}

impl User {
    fn new(name: String, age: u8) -> Self {
        Self {
            name,
            age,
            active: true,
        }
    }

    fn display(&self) -> String {
        format!(
            "{} (age {}), active={}",
            self.name, self.age, self.active
        )
    }
}

fn main() {
    let user = User::new(String::from("Ada"), 36);
    println!("{}", user.display());
}
