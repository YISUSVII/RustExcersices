trait Describable {
    fn describe(&self) -> String;
}

impl Describable for i32 {
    fn describe(&self) -> String {
        let _ = self;
        unimplemented!("Implement i32 describe")
    }
}

impl Describable for String {
    fn describe(&self) -> String {
        let _ = self;
        unimplemented!("Implement String describe")
    }
}

fn catalog(items: &[Box<dyn Describable>]) -> Vec<String> {
    let _ = items;
    unimplemented!("Implement catalog")
}

fn main() {
    let items: Vec<Box<dyn Describable>> = vec![Box::new(7), Box::new(String::from("rust"))];
    println!("{:?}", catalog(&items));
}
