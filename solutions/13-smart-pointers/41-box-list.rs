enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn sum(list: &List) -> i32 {
    match list {
        List::Nil => 0,
        List::Cons(v, next) => v + sum(next),
    }
}

fn from_slice(items: &[i32]) -> List {
    let mut list = List::Nil;
    for &item in items.iter().rev() {
        list = List::Cons(item, Box::new(list));
    }
    list
}

fn main() {
    let list = from_slice(&[1, 2, 3]);
    println!("sum={}", sum(&list));
}
