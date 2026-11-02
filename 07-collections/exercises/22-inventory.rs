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
        // TODO: implement
        todo!("Return an empty inventory")
    }

    fn add(&mut self, name: &str, quantity: u32, price_cents: u64) {
        // TODO: implement
        let _ = (name, quantity, price_cents);
        todo!("Use entry().or_insert(...) then bump quantity and set the price")
    }

    fn remove(&mut self, name: &str, quantity: u32) -> Result<u32, String> {
        // TODO: implement
        let _ = (name, quantity);
        todo!("get_mut + ok_or_else, check stock, drop the entry at 0")
    }

    fn total_value(&self) -> u64 {
        // TODO: implement
        todo!("Sum quantity * price_cents over all items")
    }

    fn low_stock(&self, threshold: u32) -> Vec<&str> {
        // TODO: implement
        let _ = threshold;
        todo!("Names with quantity < threshold, sorted")
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
