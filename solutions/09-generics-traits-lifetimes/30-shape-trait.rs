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

#[derive(Debug)]
struct Square(Rectangle);

fn ensure_positive(value: f64, what: &str) -> Result<(), String> {
    if value > 0.0 && value.is_finite() {
        Ok(())
    } else {
        Err(format!("{what} must be a positive number, got {value}"))
    }
}

impl Circle {
    fn new(radius: f64) -> Result<Self, String> {
        ensure_positive(radius, "radius")?;
        Ok(Self { radius })
    }
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Result<Self, String> {
        ensure_positive(width, "width")?;
        ensure_positive(height, "height")?;
        Ok(Self { width, height })
    }
}

impl Triangle {
    fn new(a: f64, b: f64, c: f64) -> Result<Self, String> {
        for (side, name) in [(a, "a"), (b, "b"), (c, "c")] {
            ensure_positive(side, name)?;
        }
        if a + b <= c || a + c <= b || b + c <= a {
            return Err(format!(
                "sides {a}, {b}, {c} violate the triangle inequality"
            ));
        }
        Ok(Self { a, b, c })
    }
}

impl Square {
    fn new(side: f64) -> Result<Self, String> {
        Rectangle::new(side, side).map(Square)
    }
}

impl Shape for Circle {
    fn name(&self) -> &'static str {
        "Circle"
    }
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

impl Shape for Rectangle {
    fn name(&self) -> &'static str {
        "Rectangle"
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Shape for Triangle {
    fn name(&self) -> &'static str {
        "Triangle"
    }
    fn area(&self) -> f64 {
        let s = self.perimeter() / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

impl Shape for Square {
    fn name(&self) -> &'static str {
        "Square"
    }
    fn area(&self) -> f64 {
        self.0.area()
    }
    fn perimeter(&self) -> f64 {
        self.0.perimeter()
    }
}

fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn largest_shape(shapes: &[Box<dyn Shape>]) -> Option<&dyn Shape> {
    shapes
        .iter()
        .max_by(|a, b| a.area().total_cmp(&b.area()))
        .map(|boxed| boxed.as_ref())
}

fn describe<S>(shape: &S) -> String
where
    S: Shape + ?Sized,
{
    format!(
        "{}: area={:.2}, perimeter={:.2}",
        shape.name(),
        shape.area(),
        shape.perimeter()
    )
}

fn build_shapes() -> Result<Vec<Box<dyn Shape>>, String> {
    Ok(vec![
        Box::new(Circle::new(1.0)?),
        Box::new(Rectangle::new(2.0, 3.0)?),
        Box::new(Triangle::new(3.0, 4.0, 5.0)?),
        Box::new(Square::new(2.5)?),
    ])
}

fn main() {
    let mut shapes = match build_shapes() {
        Ok(shapes) => shapes,
        Err(e) => {
            eprintln!("could not build shapes: {e}");
            return;
        }
    };

    for shape in &shapes {
        println!("{}", describe(shape.as_ref()));
    }
    println!("Total area: {:.2}", total_area(&shapes));
    if let Some(big) = largest_shape(&shapes) {
        println!("Largest: {big:?}");
    }

    shapes.sort_by(|a, b| b.area().total_cmp(&a.area()));
    let order: Vec<&str> = shapes.iter().map(|s| s.name()).collect();
    println!("Sorted by area (desc): {order:?}");

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
    fn aggregates_over_trait_objects() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Rectangle::new(1.0, 1.0).unwrap()),
            Box::new(Rectangle::new(2.0, 5.0).unwrap()),
        ];
        assert!(approx(total_area(&shapes), 11.0));
        assert!(approx(largest_shape(&shapes).unwrap().area(), 10.0));
        assert!(largest_shape(&[]).is_none());
    }

    #[test]
    fn describe_formats_two_decimals() {
        let r = Rectangle::new(2.0, 3.0).unwrap();
        assert_eq!(describe(&r), "Rectangle: area=6.00, perimeter=10.00");
    }
}
