struct User {
    name: String,
    age: u8,
}

fn names_at_least(users: &[User], min_age: u8) -> Vec<String> {
    users
        .iter()
        .filter(|u| u.age >= min_age)
        .map(|u| u.name.clone())
        .collect()
}

fn main() {
    let users = [
        User { name: "Ada".into(), age: 36 },
        User { name: "Bea".into(), age: 15 },
        User { name: "Cam".into(), age: 22 },
    ];
    println!("{:?}", names_at_least(&users, 18));
}
