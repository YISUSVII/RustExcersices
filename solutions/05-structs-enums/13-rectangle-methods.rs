struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let a = Rectangle {
        width: 10.0,
        height: 5.0,
    };
    let b = Rectangle {
        width: 4.0,
        height: 3.0,
    };
    println!("area={}", a.area());
    println!("perimeter={}", a.perimeter());
    println!("can_hold={}", a.can_hold(&b));
}
