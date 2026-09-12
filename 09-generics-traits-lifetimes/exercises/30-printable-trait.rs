trait Printable {
    fn brief(&self) -> String;
}

struct Book {
    title: String,
    pages: u32,
}

impl Printable for Book {
    fn brief(&self) -> String {
        let _ = self;
        unimplemented!("Implement Book::brief")
    }
}

fn show(item: &impl Printable) {
    // TODO: print item.brief()
    let _ = item;
    unimplemented!("Implement show")
}

fn main() {
    let book = Book {
        title: String::from("Rust"),
        pages: 500,
    };
    show(&book);
}
