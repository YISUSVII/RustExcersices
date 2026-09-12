fn add(a: i32, b: i32) -> i32 {
    // TODO: implement
    let _ = (a, b);
    unimplemented!("Implement add")
}

fn sub(a: i32, b: i32) -> i32 {
    // TODO: implement
    let _ = (a, b);
    unimplemented!("Implement sub")
}

fn mul(a: i32, b: i32) -> i32 {
    // TODO: implement
    let _ = (a, b);
    unimplemented!("Implement mul")
}

fn div(a: i32, b: i32) -> Option<i32> {
    // TODO: return None when b == 0
    let _ = (a, b);
    unimplemented!("Implement div")
}

fn calculate(a: i32, b: i32, op: char) -> Option<i32> {
    // TODO: route operators to add/sub/mul/div functions
    // Suggested behavior for operators:
    // '+' => Some(add(a, b))
    // '-' => Some(sub(a, b))
    // '*' => Some(mul(a, b))
    // '/' => div(a, b)
    let _ = (a, b, op);
    unimplemented!("Implement calculate using reusable functions")
}

fn main() {
    println!("{:?}", calculate(8, 2, '/'));
}
