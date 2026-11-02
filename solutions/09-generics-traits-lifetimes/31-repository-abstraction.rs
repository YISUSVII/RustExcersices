use std::collections::hash_map::Entry;
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
        match self {
            RepoError::NotFound(id) => write!(f, "entity with id {id} was not found"),
            RepoError::AlreadyExists(id) => write!(f, "entity with id {id} already exists"),
        }
    }
}

impl std::error::Error for RepoError {}

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
        match self.items.entry(item.id()) {
            Entry::Occupied(e) => Err(RepoError::AlreadyExists(*e.key())),
            Entry::Vacant(e) => {
                e.insert(item);
                Ok(())
            }
        }
    }

    fn get(&self, id: u32) -> Option<&T> {
        self.items.get(&id)
    }

    fn update(&mut self, item: T) -> Result<T, RepoError> {
        let slot = self
            .items
            .get_mut(&item.id())
            .ok_or(RepoError::NotFound(item.id()))?;
        Ok(std::mem::replace(slot, item))
    }

    fn delete(&mut self, id: u32) -> Result<T, RepoError> {
        self.items.remove(&id).ok_or(RepoError::NotFound(id))
    }

    fn all(&self) -> Vec<&T> {
        let mut items: Vec<&T> = self.items.values().collect();
        items.sort_by_key(|item| item.id());
        items
    }

    // Override the default: no need to build a Vec just to count.
    fn count(&self) -> usize {
        self.items.len()
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
    repo.all()
        .into_iter()
        .filter(|item| predicate(item))
        .collect()
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

fn run() -> Result<(), RepoError> {
    let mut users = InMemoryRepository::new();
    users.insert(user(2, "Bob"))?;
    users.insert(user(1, "Ada"))?;

    if let Err(e) = users.insert(user(1, "Duplicate")) {
        println!("Expected error: {e}");
    }

    let old = users.update(user(2, "Robert"))?;
    println!(
        "Renamed {} -> {:?}",
        old.name,
        users.get(2).map(|u| &u.name)
    );
    println!("All users: {:?}", users.all());
    println!("get(99): {:?}", users.get(99));
    println!("delete(99): {:?}", users.delete(99));

    let mut products = InMemoryRepository::new();
    products.insert(product(10, "Keyboard", 4_999))?;
    products.insert(product(11, "Sticker", 199))?;
    products.insert(product(12, "Mug", 899))?;

    let cheap = find_where(&products, |p: &Product| p.price_cents < 1_000);
    let titles: Vec<&str> = cheap.iter().map(|p| p.title.as_str()).collect();
    println!("Cheap products: {titles:?}");

    let removed = products.delete(10)?;
    println!(
        "Removed {}; {} products left",
        removed.title,
        products.count()
    );
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
    }
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

    #[test]
    fn find_where_filters_generically() {
        let mut repo = InMemoryRepository::new();
        repo.insert(product(1, "Cheap", 50)).unwrap();
        repo.insert(product(2, "Pricey", 5_000)).unwrap();
        let found = find_where(&repo, |p: &Product| p.price_cents < 100);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].title, "Cheap");
    }
}
