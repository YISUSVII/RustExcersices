#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64, f64),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn area(shape: &Shape) -> f64 {
    // TODO: implement
    let _ = shape;
    todo!("Destructure each Shape variant; use Heron's formula for Triangle")
}

fn describe(msg: &Message) -> String {
    // TODO: implement
    let _ = msg;
    todo!("Use literal sub-patterns (x: 0), guards (if ...) and specific-first ordering")
}

fn classify_age(age: u8) -> String {
    // TODO: implement
    let _ = age;
    todo!("Use n @ 0..=12 style bindings")
}

fn main() {
    println!("circle area: {:.2}", area(&Shape::Circle { radius: 1.0 }));
    println!(
        "rect area: {:.2}",
        area(&Shape::Rectangle {
            width: 2.0,
            height: 3.0
        })
    );
    println!(
        "triangle area: {:.2}",
        area(&Shape::Triangle(3.0, 4.0, 5.0))
    );

    let messages = [
        Message::Move { x: 0, y: 0 },
        Message::Move { x: 5, y: 0 },
        Message::Write("hi".to_string()),
        Message::ChangeColor(128, 128, 128),
        Message::ChangeColor(255, 128, 0),
        Message::Quit,
    ];
    for msg in &messages {
        println!("{}", describe(msg));
    }
    println!("{}", classify_age(15));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn areas() {
        assert!((area(&Shape::Circle { radius: 2.0 }) - 12.566).abs() < 0.001);
        assert_eq!(
            area(&Shape::Rectangle {
                width: 2.0,
                height: 5.0
            }),
            10.0
        );
        assert!((area(&Shape::Triangle(3.0, 4.0, 5.0)) - 6.0).abs() < 1e-9);
    }

    #[test]
    fn move_patterns() {
        assert_eq!(describe(&Message::Move { x: 0, y: 0 }), "stay");
        assert_eq!(
            describe(&Message::Move { x: 0, y: -2 }),
            "move vertically by -2"
        );
        assert_eq!(describe(&Message::Move { x: 1, y: 2 }), "move to (1, 2)");
    }

    #[test]
    fn guards() {
        assert_eq!(describe(&Message::Write("   ".into())), "empty message");
        assert_eq!(describe(&Message::ChangeColor(0, 0, 0)), "grayscale 0");
        assert_eq!(
            describe(&Message::ChangeColor(255, 128, 0)),
            "color #FF8000"
        );
        assert_eq!(describe(&Message::Quit), "quit");
    }

    #[test]
    fn age_bindings() {
        assert_eq!(classify_age(5), "child (5)");
        assert_eq!(classify_age(19), "teen (19)");
        assert_eq!(classify_age(20), "adult (20)");
    }
}
