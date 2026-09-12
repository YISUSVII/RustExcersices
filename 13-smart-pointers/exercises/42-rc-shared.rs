use std::rc::Rc;

fn attach_label(base: &Rc<String>) -> Rc<String> {
    // TODO: return another owner of the same allocation
    let _ = base;
    unimplemented!("Implement attach_label")
}

fn main() {
    let label = Rc::new(String::from("shared"));
    println!("count={}", Rc::strong_count(&label));
    let other = attach_label(&label);
    println!("count={}", Rc::strong_count(&label));
    println!("{label} / {other}");
}
