enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn describe(msg: &Message) -> String {
    match msg {
        Message::Quit => String::from("quit"),
        Message::Move { x, y } => format!("move to ({x}, {y})"),
        Message::Write(text) => format!("write: {text}"),
        Message::ChangeColor(r, g, b) => format!("color rgb({r}, {g}, {b})"),
    }
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
