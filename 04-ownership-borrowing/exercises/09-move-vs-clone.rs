fn print_message(message: String) {
    println!("{message}");
}

fn main() {
    let msg = String::from("Ownership matters");
    print_message(msg);
    // Intentional broken line for learning (E0382):
    println!("Again: {msg}");
}
