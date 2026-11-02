use std::f64::consts::PI;
use std::fmt::Debug;

trait Shape: Debug {
    fn name(&self) -> &'static str;
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

#[derive(Debug)]
struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Circle {
    fn new(radius: f64) -> Result<Self, String> {
        // TODO: validate radius > 0
        Ok(Self { radius })
    }
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Result<Self, String> {
        // TODO: validate both sides > 0
        Ok(Self { width, height })
    }
}

impl Triangle {
    fn new(a: f64, b: f64, c: f64) -> Result<Self, String> {
        // TODO: validate sides > 0 and the triangle inequality
        Ok(Self { a, b, c })
    }
}

impl Shape for Circle {
    fn name(&self) -> &'static str {
        "Circle"
    }
    fn area(&self) -> f64 {
        // TODO: implement
        let _ = (PI, self.radius);
        todo!("PI * r * r")
    }
    fn perimeter(&self) -> f64 {
        // TODO: implement
        todo!("2 * PI * r")
    }
}

impl Shape for Rectangle {
    fn name(&self) -> &'static str {
        "Rectangle"
    }
    fn area(&self) -> f64 {
        // TODO: implement
        let _ = (self.width, self.height);
        todo!("width * height")
    }
    fn perimeter(&self) -> f64 {
        // TODO: implement
        todo!("2 * (width + height)")
    }
}

impl Shape for Triangle {
    fn name(&self) -> &'static str {
        "Triangle"
    }
    fn area(&self) -> f64 {
        // TODO: implement with Heron's formula
        let _ = (self.a, self.b, self.c);
        todo!("s = perimeter / 2; sqrt(s(s-a)(s-b)(s-c))")
    }
    fn perimeter(&self) -> f64 {
        // TODO: implement
        todo!("a + b + c")
    }
}

fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    // TODO: implement
    let _ = shapes;
    todo!("Sum the area of every shape")
}

fn largest_shape(shapes: &[Box<dyn Shape>]) -> Option<&dyn Shape> {
    // TODO: implement
    let _ = shapes;
    todo!("Find the shape with the biggest area (hint: f64::total_cmp)")
}

fn describe<S>(shape: &S) -> String
where
    S: Shape + ?Sized,
{
    // TODO: implement
    let _ = shape;
    todo!("\"<name>: area=<a>, perimeter=<p>\" with 2 decimals")
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle::new(1.0).expect("valid circle")),
        Box::new(Rectangle::new(2.0, 3.0).expect("valid rectangle")),
        Box::new(Triangle::new(3.0, 4.0, 5.0).expect("valid triangle")),
    ];

    for shape in &shapes {
        println!("{}", describe(shape.as_ref()));
    }
    println!("Total area: {:.2}", total_area(&shapes));
    if let Some(big) = largest_shape(&shapes) {
        println!("Largest: {big:?}");
    }

    println!(
        "Invalid triangle: {:?}",
        Triangle::new(1.0, 2.0, 10.0).err()
    );
    println!("Invalid circle: {:?}", Circle::new(-1.0).err());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn areas_and_perimeters() {
        assert!(approx(Circle::new(1.0).unwrap().area(), PI));
        assert!(approx(Rectangle::new(2.0, 3.0).unwrap().perimeter(), 10.0));
        assert!(approx(Triangle::new(3.0, 4.0, 5.0).unwrap().area(), 6.0));
    }

    #[test]
    fn constructors_validate_input() {
        assert!(Circle::new(-1.0).is_err());
        assert!(Rectangle::new(0.0, 2.0).is_err());
        let err = Triangle::new(1.0, 2.0, 10.0).unwrap_err();
        assert!(
            err.contains("triangle inequality"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn describe_formats_two_decimals() {
        let r = Rectangle::new(2.0, 3.0).unwrap();
        assert_eq!(describe(&r), "Rectangle: area=6.00, perimeter=10.00");
    }
}
