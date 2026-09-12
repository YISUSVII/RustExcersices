fn print_message(message: &str) {
    println!("{message}");
}

fn main() {
    let msg = String::from("Ownership matters");
    print_message(&msg);
    println!("Again: {msg}");
}
