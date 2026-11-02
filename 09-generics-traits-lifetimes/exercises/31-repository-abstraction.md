# Exercise 31 — Repository Abstraction

- **Title:** Repository Abstraction
- **Objective:** Design a generic `Repository<T>` trait and an in-memory implementation that works for any entity type with an id.
- **Difficulty:** Advanced
- **Concepts practiced:** generic traits, associated types vs generic parameters, trait bounds, `HashMap`, `Result` with a custom error enum, borrowing in return types

## Instructions

1. Define an `Entity` trait for things that can be stored:
   ```rust
   trait Entity: Clone {
       fn id(&self) -> u32;
   }
   ```
2. Define a custom error type:
   ```rust
   #[derive(Debug, PartialEq)]
   enum RepoError { NotFound(u32), AlreadyExists(u32) }
   ```
   and implement `std::fmt::Display` for it.
3. Define the generic trait:
   ```rust
   trait Repository<T: Entity> {
       fn insert(&mut self, item: T) -> Result<(), RepoError>;
       fn get(&self, id: u32) -> Option<&T>;
       fn update(&mut self, item: T) -> Result<T, RepoError>; // returns the old value
       fn delete(&mut self, id: u32) -> Result<T, RepoError>;
       fn all(&self) -> Vec<&T>;                              // sorted by id
       fn count(&self) -> usize { self.all().len() }          // default method
   }
   ```
4. Implement `InMemoryRepository<T>` backed by `HashMap<u32, T>` with `InMemoryRepository::new()`, and implement `Repository<T>` for it for every `T: Entity`.
5. Create two entity types, `User { id, name }` and `Product { id, title, price_cents }`, and use the **same** repository code for both.
6. Write a generic helper `find_where<T, R, P>(repo: &R, predicate: P) -> Vec<&T>` with a `where` clause (`T: Entity`, `R: Repository<T>`, `P: Fn(&T) -> bool`).

## Expected behavior

- Inserting a user twice with id `1` => `Err(RepoError::AlreadyExists(1))`
- `get(99)` => `None`
- `delete(99)` => `Err(RepoError::NotFound(99))`
- `update(user)` returns the previous version of that user
- `all()` returns items ordered by id
- `find_where(&products, |p| p.price_cents < 1000)` returns only cheap products

## Optional hints

- `HashMap::entry(id)` with `Entry::Occupied` / `Entry::Vacant` makes `insert` clean.
- `HashMap::insert` returns the old value — handy for `update` once you've checked the key exists.
- `ok_or(RepoError::NotFound(id))` turns an `Option` into a `Result`.

## Bonus challenge

Make the id type generic too: `trait Entity { type Id: Eq + Hash + Ord + Copy; fn id(&self) -> Self::Id; }` and adapt the repository.
