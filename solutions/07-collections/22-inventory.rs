use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct Item {
    quantity: u32,
    price_cents: u64,
}

#[derive(Debug, Default)]
struct Inventory {
    items: HashMap<String, Item>,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory::default()
    }

    fn add(&mut self, name: &str, quantity: u32, price_cents: u64) {
        let item = self.items.entry(name.to_string()).or_insert(Item {
            quantity: 0,
            price_cents,
        });
        item.quantity += quantity;
        item.price_cents = price_cents;
    }

    fn remove(&mut self, name: &str, quantity: u32) -> Result<u32, String> {
        let item = self
            .items
            .get_mut(name)
            .ok_or_else(|| format!("unknown item: {name}"))?;
        if quantity > item.quantity {
            return Err(format!(
                "not enough {name}: have {}, requested {quantity}",
                item.quantity
            ));
        }
        item.quantity -= quantity;
        let remaining = item.quantity;
        if remaining == 0 {
            self.items.remove(name);
        }
        Ok(remaining)
    }

    fn total_value(&self) -> u64 {
        self.items
            .values()
            .map(|item| u64::from(item.quantity) * item.price_cents)
            .sum()
    }

    fn low_stock(&self, threshold: u32) -> Vec<&str> {
        let mut names: Vec<&str> = self
            .items
            .iter()
            .filter(|(_, item)| item.quantity < threshold)
            .map(|(name, _)| name.as_str())
            .collect();
        names.sort_unstable();
        names
    }
}

fn format_cents(cents: u64) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}

fn main() {
    let mut inv = Inventory::new();
    inv.add("apple", 10, 50);
    inv.add("banana", 6, 25);
    inv.add("pear", 4, 80);
    inv.add("orange", 12, 40);

    println!("remove 3 apples -> {:?}", inv.remove("apple", 3));
    println!("remove 5 kiwis -> {:?}", inv.remove("kiwi", 5));
    println!("remove 9 pears -> {:?}", inv.remove("pear", 9));
    println!("remove 4 pears -> {:?}", inv.remove("pear", 4));
    println!("total value: {}", format_cents(inv.total_value()));
    println!("low stock (< 8): {:?}", inv.low_stock(8));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_accumulates_quantity() {
        let mut inv = Inventory::new();
        inv.add("x", 2, 100);
        inv.add("x", 3, 120);
        assert_eq!(
            inv.items["x"],
            Item {
                quantity: 5,
                price_cents: 120
            }
        );
    }

    #[test]
    fn remove_errors_and_cleanup() {
        let mut inv = Inventory::new();
        inv.add("x", 2, 100);
        assert!(inv.remove("y", 1).is_err());
        assert!(inv.remove("x", 3).is_err());
        assert_eq!(inv.remove("x", 2), Ok(0));
        assert!(!inv.items.contains_key("x"));
    }

    #[test]
    fn value_and_low_stock() {
        let mut inv = Inventory::new();
        inv.add("b", 1, 1000);
        inv.add("a", 2, 250);
        inv.add("c", 50, 1);
        assert_eq!(inv.total_value(), 1550);
        assert_eq!(inv.low_stock(5), vec!["a", "b"]);
        assert_eq!(format_cents(1550), "$15.50");
    }
}
