trait Container {
    type Item;
    fn first(&self) -> Option<&Self::Item>;
}

impl<T> Container for Vec<T> {
    type Item = T;
    fn first(&self) -> Option<&Self::Item> {
        self.get(0)
    }
}

fn show_first<C: Container>(c: &C)
where
    C::Item: std::fmt::Display,
{
    match c.first() {
        Some(v) => println!("first={v}"),
        None => println!("empty"),
    }
}

fn main() {
    let v = vec![10, 20, 30];
    show_first(&v);
}
