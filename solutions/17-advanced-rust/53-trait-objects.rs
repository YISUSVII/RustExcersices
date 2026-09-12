trait Describable {
    fn describe(&self) -> String;
}

impl Describable for i32 {
    fn describe(&self) -> String {
        format!("i32:{self}")
    }
}

impl Describable for String {
    fn describe(&self) -> String {
        format!("String:{self}")
    }
}

fn catalog(items: &[Box<dyn Describable>]) -> Vec<String> {
    items.iter().map(|i| i.describe()).collect()
}

fn main() {
    let items: Vec<Box<dyn Describable>> = vec![Box::new(7), Box::new(String::from("rust"))];
    println!("{:?}", catalog(&items));
}
