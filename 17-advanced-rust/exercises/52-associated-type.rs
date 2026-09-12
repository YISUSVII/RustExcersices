trait Container {
    type Item;
    fn first(&self) -> Option<&Self::Item>;
}

impl<T> Container for Vec<T> {
    type Item = T;
    fn first(&self) -> Option<&Self::Item> {
        let _ = self;
        unimplemented!("Implement Vec::first")
    }
}

fn show_first<C: Container>(c: &C)
where
    C::Item: std::fmt::Display,
{
    let _ = c;
    unimplemented!("Implement show_first")
}

fn main() {
    let v = vec![10, 20, 30];
    show_first(&v);
}
