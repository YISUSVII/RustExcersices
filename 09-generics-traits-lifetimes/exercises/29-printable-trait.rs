trait Printable {
    fn label(&self) -> String;

    fn print(&self) -> String {
        // TODO: implement the default method
        todo!("Wrap self.label() in square brackets")
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
        // TODO: implement
        let _ = (&self.title, &self.author);
        todo!("\"Book: <title> by <author>\"")
    }
}

impl Printable for Temperature {
    fn label(&self) -> String {
        // TODO: implement
        let _ = self.celsius;
        todo!("\"Temp: <celsius>°C\" with 1 decimal place")
    }

    // TODO: override `print` so it returns "~<label>~"
}

impl Printable for i32 {
    fn label(&self) -> String {
        // TODO: implement
        todo!("\"Number: <n>\"")
    }
}

fn print_one<T: Printable>(item: &T) -> String {
    // TODO: implement
    let _ = item;
    todo!("Call print() through the trait bound")
}

fn print_all(items: &[&dyn Printable]) -> Vec<String> {
    // TODO: implement
    let _ = items;
    todo!("Map each trait object to its print() output")
}

fn make_printable(n: i32) -> impl Printable {
    // TODO: return any value whose type implements Printable
    n
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
}
