use std::rc::Rc;

fn attach_label(base: &Rc<String>) -> Rc<String> {
    Rc::clone(base)
}

fn main() {
    let label = Rc::new(String::from("shared"));
    println!("count={}", Rc::strong_count(&label));
    let other = attach_label(&label);
    println!("count={}", Rc::strong_count(&label));
    println!("{label} / {other}");
}
