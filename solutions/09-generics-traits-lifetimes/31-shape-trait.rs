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
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}

fn total_area(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn main() {
    let c = Circle { radius: 1.0 };
    let r = Rectangle { w: 2.0, h: 3.0 };
    let shapes: [&dyn Shape; 2] = [&c, &r];
    println!("{}", total_area(&shapes));
}
