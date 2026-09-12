enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn describe(msg: &Message) -> String {
    // TODO: match each variant and build a description
    let _ = msg;
    unimplemented!("Implement describe")
}

fn main() {
    let messages = [
        Message::Quit,
        Message::Move { x: 2, y: 3 },
        Message::Write(String::from("hi")),
        Message::ChangeColor(1, 2, 3),
    ];
    for m in &messages {
        println!("{}", describe(m));
    }
}
