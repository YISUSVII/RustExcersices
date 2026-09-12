trait Printable {
    fn brief(&self) -> String;
}

struct Book {
    title: String,
    pages: u32,
}

impl Printable for Book {
    fn brief(&self) -> String {
        format!("'{}' ({} pages)", self.title, self.pages)
    }
}

fn show(item: &impl Printable) {
    println!("{}", item.brief());
}

fn main() {
    let book = Book {
        title: String::from("Rust"),
        pages: 500,
    };
    show(&book);
}
