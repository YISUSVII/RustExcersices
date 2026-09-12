trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    w: f64,
    h: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        let _ = self;
        unimplemented!("Implement Circle::area")
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        let _ = self;
        unimplemented!("Implement Rectangle::area")
    }
}

fn total_area(shapes: &[&dyn Shape]) -> f64 {
    let _ = shapes;
    unimplemented!("Implement total_area")
}

fn main() {
    let c = Circle { radius: 1.0 };
    let r = Rectangle { w: 2.0, h: 3.0 };
    let shapes: [&dyn Shape; 2] = [&c, &r];
    println!("{}", total_area(&shapes));
}
