trait Printable {
    fn label(&self) -> String;

    fn print(&self) -> String {
        format!("[{}]", self.label())
    }
}

struct Book {
    title: String,
    author: String,
}

struct Temperature {
    celsius: f64,
}

impl Printable for Book {
    fn label(&self) -> String {
        format!("Book: {} by {}", self.title, self.author)
    }
}

impl Printable for Temperature {
    fn label(&self) -> String {
        format!("Temp: {:.1}°C", self.celsius)
    }

    fn print(&self) -> String {
        format!("~{}~", self.label())
    }
}

impl Printable for i32 {
    fn label(&self) -> String {
        format!("Number: {self}")
    }
}

// Bonus: a Vec of printables is printable too.
impl<T: Printable> Printable for Vec<T> {
    fn label(&self) -> String {
        self.iter()
            .map(Printable::label)
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn print_one<T: Printable>(item: &T) -> String {
    item.print()
}

fn print_all(items: &[&dyn Printable]) -> Vec<String> {
    items.iter().map(|item| item.print()).collect()
}

fn make_printable(n: i32) -> impl Printable {
    n * 2
}

fn main() {
    let book = Book {
        title: String::from("Dune"),
        author: String::from("Herbert"),
    };
    let temp = Temperature { celsius: 21.456 };

    println!("{}", print_one(&book));
    println!("{}", print_one(&temp));
    println!("{}", print_one(&42));

    for line in print_all(&[&book, &temp, &7]) {
        println!("dyn -> {line}");
    }

    println!("impl Trait -> {}", make_printable(21).print());
    println!("Vec -> {}", vec![1, 2, 3].print());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_print_wraps_label_in_brackets() {
        let book = Book {
            title: "Dune".into(),
            author: "Herbert".into(),
        };
        assert_eq!(book.print(), "[Book: Dune by Herbert]");
        assert_eq!(42.print(), "[Number: 42]");
    }

    #[test]
    fn overridden_print_is_used() {
        let t = Temperature { celsius: 21.456 };
        assert_eq!(t.print(), "~Temp: 21.5°C~");
    }

    #[test]
    fn print_all_preserves_order_with_trait_objects() {
        let t = Temperature { celsius: 0.0 };
        let out = print_all(&[&1, &t]);
        assert_eq!(out, vec!["[Number: 1]", "~Temp: 0.0°C~"]);
    }

    #[test]
    fn blanket_vec_impl_joins_labels() {
        assert_eq!(vec![1, 2].print(), "[Number: 1, Number: 2]");
    }
}
