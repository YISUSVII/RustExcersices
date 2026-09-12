enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn sum(list: &List) -> i32 {
    let _ = list;
    unimplemented!("Implement sum")
}

fn from_slice(items: &[i32]) -> List {
    let _ = items;
    unimplemented!("Implement from_slice")
}

fn main() {
    let list = from_slice(&[1, 2, 3]);
    println!("sum={}", sum(&list));
}
