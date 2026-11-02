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
    users.iter().filter(|u| u.active && u.age >= 18).collect()
}

fn names_with_role<'a>(users: &'a [User], role: &Role) -> Vec<&'a str> {
    let mut names: Vec<&str> = users
        .iter()
        .filter(|u| &u.role == role)
        .map(|u| u.name.as_str())
        .collect();
    names.sort_unstable();
    names
}

fn oldest(users: &[User]) -> Option<&User> {
    users.iter().max_by_key(|u| u.age)
}

fn split_by_active(users: &[User]) -> (Vec<&User>, Vec<&User>) {
    users.iter().partition(|u| u.active)
}

fn group_by_role(users: &[User]) -> BTreeMap<String, Vec<&str>> {
    users.iter().fold(BTreeMap::new(), |mut groups, u| {
        groups
            .entry(format!("{:?}", u.role))
            .or_insert_with(Vec::new)
            .push(u.name.as_str());
        groups
    })
}

fn has_admin(users: &[User]) -> bool {
    users.iter().any(|u| u.role == Role::Admin)
}

fn all_named(users: &[User]) -> bool {
    users.iter().all(|u| !u.name.trim().is_empty())
}

fn min_age(age: u32) -> UserFilter {
    Box::new(move |u| u.age >= age)
}

fn with_role(role: Role) -> UserFilter {
    Box::new(move |u| u.role == role)
}

fn name_contains(needle: &str) -> UserFilter {
    let needle = needle.to_lowercase();
    Box::new(move |u| u.name.to_lowercase().contains(&needle))
}

fn and(a: UserFilter, b: UserFilter) -> UserFilter {
    Box::new(move |u| a(u) && b(u))
}

fn not(f: UserFilter) -> UserFilter {
    Box::new(move |u| !f(u))
}

fn apply_filters<'a>(users: &'a [User], filters: &[UserFilter]) -> Vec<&'a User> {
    users
        .iter()
        .filter(|u| filters.iter().all(|f| f(u)))
        .collect()
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

    let filters = vec![min_age(30), with_role(Role::Editor)];
    println!("30+ editors: {:?}", names(&apply_filters(&users, &filters)));

    let combo = vec![and(name_contains("a"), not(with_role(Role::Admin)))];
    println!(
        "'a' in name, not admin: {:?}",
        names(&apply_filters(&users, &combo))
    );
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
        let found = apply_filters(&users, &[not(name_contains("AN"))]);
        assert_eq!(names(&found), vec!["Ben", "Cleo"]);
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
