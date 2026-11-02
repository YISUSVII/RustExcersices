# Exercise 34 — User Filtering

- **Title:** User Filtering
- **Objective:** Query a list of users with iterator chains and reusable, composable closure-based filters.
- **Difficulty:** Intermediate
- **Concepts practiced:** iterators over structs, borrowing in closures, `filter`, `map`, `any`/`all`, `partition`, `max_by_key`, `sort_by`, grouping with `HashMap`/`BTreeMap`, boxed closures (`Box<dyn Fn>`)

## Instructions

Given:

```rust
#[derive(Debug, Clone, PartialEq)]
enum Role { Admin, Editor, Viewer }

#[derive(Debug, Clone, PartialEq)]
struct User { name: String, age: u32, role: Role, active: bool }
```

Implement:

1. `active_adults(users: &[User]) -> Vec<&User>` — active users aged 18+.
2. `names_with_role(users: &[User], role: &Role) -> Vec<&str>` — names sorted alphabetically.
3. `oldest(users: &[User]) -> Option<&User>`.
4. `split_by_active(users: &[User]) -> (Vec<&User>, Vec<&User>)` — `(active, inactive)` using `partition`.
5. `group_by_role(users: &[User]) -> BTreeMap<String, Vec<&str>>` — key is `format!("{:?}", role)`.
6. `has_admin(users: &[User]) -> bool` and `all_named(users: &[User]) -> bool` (no empty names).
7. Composable filters:
   - `type UserFilter = Box<dyn Fn(&User) -> bool>;`
   - `min_age(age: u32) -> UserFilter`, `with_role(role: Role) -> UserFilter`, `name_contains(needle: &str) -> UserFilter`
   - `apply_filters<'a>(users: &'a [User], filters: &[UserFilter]) -> Vec<&'a User>` — keep users that match **all** filters.

## Expected behavior

With sample users Ana (34, Admin, active), Ben (17, Viewer, active), Cleo (25, Editor, inactive), Dan (41, Editor, active):

- `active_adults` => Ana, Dan
- `names_with_role(&users, &Role::Editor)` => `["Cleo", "Dan"]`
- `oldest` => Dan
- `split_by_active` => 3 active, 1 inactive
- `group_by_role` => `{"Admin": ["Ana"], "Editor": ["Cleo", "Dan"], "Viewer": ["Ben"]}`
- `apply_filters(&users, &[min_age(30), with_role(Role::Editor)])` => Dan

## Optional hints

- Return `Vec<&User>` to avoid cloning; the borrow checker ties the result to `users`.
- `name_contains` must own its needle (`needle.to_lowercase()`) because the closure outlives the argument.
- `filters.iter().all(|f| f(user))`.

## Bonus challenge

Add `fn and(a: UserFilter, b: UserFilter) -> UserFilter` and `fn not(f: UserFilter) -> UserFilter` combinators.
