struct User {
    name: String,
    age: u8,
    active: bool,
}

impl User {
    fn new(name: String, age: u8) -> Self {
        // TODO: construct User with active = true
        let _ = (name, age);
        unimplemented!("Implement User::new")
    }

    fn display(&self) -> String {
        // TODO: return a human-readable summary
        let _ = self;
        unimplemented!("Implement User::display")
    }
}

fn main() {
    let user = User::new(String::from("Ada"), 36);
    println!("{}", user.display());
}
