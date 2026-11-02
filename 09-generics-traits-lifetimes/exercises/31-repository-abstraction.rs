use std::collections::HashMap;
use std::fmt;

trait Entity: Clone {
    fn id(&self) -> u32;
}

#[derive(Debug, PartialEq)]
enum RepoError {
    NotFound(u32),
    AlreadyExists(u32),
}

impl fmt::Display for RepoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: implement a friendly message for each variant
        let _ = f;
        todo!("match self and write! a message")
    }
}

trait Repository<T: Entity> {
    fn insert(&mut self, item: T) -> Result<(), RepoError>;
    fn get(&self, id: u32) -> Option<&T>;
    fn update(&mut self, item: T) -> Result<T, RepoError>;
    fn delete(&mut self, id: u32) -> Result<T, RepoError>;
    fn all(&self) -> Vec<&T>;

    fn count(&self) -> usize {
        self.all().len()
    }
}

struct InMemoryRepository<T> {
    items: HashMap<u32, T>,
}

impl<T> InMemoryRepository<T> {
    fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

impl<T: Entity> Repository<T> for InMemoryRepository<T> {
    fn insert(&mut self, item: T) -> Result<(), RepoError> {
        // TODO: implement
        let _ = (&self.items, item);
        todo!("Err(AlreadyExists) if the id is taken, otherwise store the item")
    }

    fn get(&self, id: u32) -> Option<&T> {
        // TODO: implement
        let _ = id;
        todo!("Look the id up in the HashMap")
    }

    fn update(&mut self, item: T) -> Result<T, RepoError> {
        // TODO: implement
        let _ = item;
        todo!("Replace an existing item and return the old one, or Err(NotFound)")
    }

    fn delete(&mut self, id: u32) -> Result<T, RepoError> {
        // TODO: implement
        let _ = id;
        todo!("Remove and return the item, or Err(NotFound)")
    }

    fn all(&self) -> Vec<&T> {
        // TODO: implement
        todo!("Collect references to every item, sorted by id")
    }
}

#[derive(Debug, Clone, PartialEq)]
struct User {
    id: u32,
    name: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Product {
    id: u32,
    title: String,
    price_cents: u64,
}

impl Entity for User {
    fn id(&self) -> u32 {
        self.id
    }
}

impl Entity for Product {
    fn id(&self) -> u32 {
        self.id
    }
}

fn find_where<'r, T, R, P>(repo: &'r R, predicate: P) -> Vec<&'r T>
where
    T: Entity,
    R: Repository<T>,
    P: Fn(&T) -> bool,
{
    // TODO: implement
    let _ = (repo, predicate);
    todo!("Filter repo.all() with the predicate")
}

fn user(id: u32, name: &str) -> User {
    User {
        id,
        name: name.to_string(),
    }
}

fn product(id: u32, title: &str, price_cents: u64) -> Product {
    Product {
        id,
        title: title.to_string(),
        price_cents,
    }
}

fn main() {
    let mut users = InMemoryRepository::new();
    println!("{:?}", users.insert(user(2, "Bob")));
    println!("{:?}", users.insert(user(1, "Ada")));
    println!("duplicate: {:?}", users.insert(user(1, "Duplicate")));
    println!("update: {:?}", users.update(user(2, "Robert")));
    println!("all: {:?}", users.all());
    println!("get(99): {:?}", users.get(99));
    println!("delete(99): {:?}", users.delete(99));

    let mut products = InMemoryRepository::new();
    let _ = products.insert(product(10, "Keyboard", 4_999));
    let _ = products.insert(product(11, "Sticker", 199));
    let cheap = find_where(&products, |p: &Product| p.price_cents < 1_000);
    println!("cheap: {cheap:?}");
    println!("count: {}", products.count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_rejects_duplicates() {
        let mut repo = InMemoryRepository::new();
        assert_eq!(repo.insert(user(1, "Ada")), Ok(()));
        assert_eq!(
            repo.insert(user(1, "Again")),
            Err(RepoError::AlreadyExists(1))
        );
        assert_eq!(repo.count(), 1);
    }

    #[test]
    fn update_returns_previous_value() -> Result<(), RepoError> {
        let mut repo = InMemoryRepository::new();
        repo.insert(user(1, "Ada"))?;
        let old = repo.update(user(1, "Ada L."))?;
        assert_eq!(old.name, "Ada");
        assert_eq!(repo.get(1).map(|u| u.name.as_str()), Some("Ada L."));
        assert_eq!(repo.update(user(5, "Ghost")), Err(RepoError::NotFound(5)));
        Ok(())
    }

    #[test]
    fn delete_and_all_are_ordered() -> Result<(), RepoError> {
        let mut repo = InMemoryRepository::new();
        for id in [3, 1, 2] {
            repo.insert(product(id, "x", 100))?;
        }
        let ids: Vec<u32> = repo.all().iter().map(|p| p.id).collect();
        assert_eq!(ids, vec![1, 2, 3]);
        assert_eq!(repo.delete(2)?.id, 2);
        assert_eq!(repo.delete(2), Err(RepoError::NotFound(2)));
        Ok(())
    }
}
