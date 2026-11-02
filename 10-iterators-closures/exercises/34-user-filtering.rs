use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
enum Role {
    Admin,
    Editor,
    Viewer,
}

#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: u32,
    role: Role,
    active: bool,
}

type UserFilter = Box<dyn Fn(&User) -> bool>;

fn active_adults(users: &[User]) -> Vec<&User> {
    // TODO: implement
    let _ = users;
    todo!("Active users aged 18+")
}

fn names_with_role<'a>(users: &'a [User], role: &Role) -> Vec<&'a str> {
    // TODO: implement
    let _ = (users, role);
    todo!("Names of users with this role, sorted")
}

fn oldest(users: &[User]) -> Option<&User> {
    // TODO: implement
    let _ = users;
    todo!("Use max_by_key")
}

fn split_by_active(users: &[User]) -> (Vec<&User>, Vec<&User>) {
    // TODO: implement
    let _ = users;
    todo!("Use partition")
}

fn group_by_role(users: &[User]) -> BTreeMap<String, Vec<&str>> {
    // TODO: implement
    let _ = users;
    todo!("Key = format of the role with Debug, value = names")
}

fn has_admin(users: &[User]) -> bool {
    // TODO: implement
    let _ = users;
    todo!("Use any")
}

fn all_named(users: &[User]) -> bool {
    // TODO: implement
    let _ = users;
    todo!("Use all; names must not be blank")
}

fn min_age(age: u32) -> UserFilter {
    // TODO: implement
    let _ = age;
    todo!("Box a move closure checking u.age >= age")
}

fn with_role(role: Role) -> UserFilter {
    // TODO: implement
    let _ = role;
    todo!("Box a move closure comparing roles")
}

fn name_contains(needle: &str) -> UserFilter {
    // TODO: implement (case-insensitive)
    let _ = needle;
    todo!("Own the lowercase needle, then box a move closure")
}

fn apply_filters<'a>(users: &'a [User], filters: &[UserFilter]) -> Vec<&'a User> {
    // TODO: implement
    let _ = (users, filters);
    todo!("Keep users that pass every filter")
}

fn sample_users() -> Vec<User> {
    let make = |name: &str, age, role, active| User {
        name: name.to_string(),
        age,
        role,
        active,
    };
    vec![
        make("Ana", 34, Role::Admin, true),
        make("Ben", 17, Role::Viewer, true),
        make("Cleo", 25, Role::Editor, false),
        make("Dan", 41, Role::Editor, true),
    ]
}

fn names<'a>(users: &[&'a User]) -> Vec<&'a str> {
    users.iter().map(|u| u.name.as_str()).collect()
}

fn main() {
    let users = sample_users();

    println!("active adults: {:?}", names(&active_adults(&users)));
    println!("editors: {:?}", names_with_role(&users, &Role::Editor));
    println!("oldest: {:?}", oldest(&users).map(|u| &u.name));

    let (active, inactive) = split_by_active(&users);
    println!(
        "active: {:?}, inactive: {:?}",
        names(&active),
        names(&inactive)
    );

    println!("by role: {:?}", group_by_role(&users));
    println!(
        "has admin: {}, all named: {}",
        has_admin(&users),
        all_named(&users)
    );

    let filters = vec![min_age(30), with_role(Role::Editor), name_contains("d")];
    println!("filtered: {:?}", names(&apply_filters(&users, &filters)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_queries() {
        let users = sample_users();
        assert_eq!(names(&active_adults(&users)), vec!["Ana", "Dan"]);
        assert_eq!(names_with_role(&users, &Role::Editor), vec!["Cleo", "Dan"]);
        assert_eq!(oldest(&users).map(|u| u.name.as_str()), Some("Dan"));
        assert_eq!(oldest(&[]), None);
    }

    #[test]
    fn partition_and_group() {
        let users = sample_users();
        let (active, inactive) = split_by_active(&users);
        assert_eq!((active.len(), inactive.len()), (3, 1));
        let groups = group_by_role(&users);
        assert_eq!(groups["Editor"], vec!["Cleo", "Dan"]);
        assert_eq!(groups.len(), 3);
    }

    #[test]
    fn composable_filters() {
        let users = sample_users();
        let found = apply_filters(&users, &[min_age(30), with_role(Role::Editor)]);
        assert_eq!(names(&found), vec!["Dan"]);
        assert_eq!(apply_filters(&users, &[]).len(), users.len());
    }

    #[test]
    fn any_and_all() {
        let mut users = sample_users();
        assert!(has_admin(&users) && all_named(&users));
        users[0].name = "  ".into();
        assert!(!all_named(&users));
    }
}
