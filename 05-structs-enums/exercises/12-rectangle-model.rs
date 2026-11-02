#[derive(Debug, PartialEq)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn square(size: f64) -> Rectangle {
        // TODO: implement
        let _ = size;
        todo!("Return a Rectangle whose width and height are both size")
    }

    fn area(&self) -> f64 {
        // TODO: implement
        todo!("width * height")
    }

    fn perimeter(&self) -> f64 {
        // TODO: implement
        todo!("2 * (width + height)")
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        // TODO: implement
        let _ = other;
        todo!("true when other is strictly smaller in both dimensions")
    }

    fn scale(&mut self, factor: f64) {
        // TODO: implement
        let _ = factor;
        todo!("Multiply width and height by factor")
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 10.0,
        height: 5.0,
    };
    let sq = Rectangle::square(3.0);

    println!("area: {}", rect.area());
    println!("perimeter: {}", rect.perimeter());
    println!("can hold square: {}", rect.can_hold(&sq));
    rect.scale(2.0);
    println!("scaled: {rect:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_and_perimeter() {
        let r = Rectangle {
            width: 4.0,
            height: 2.0,
        };
        assert_eq!(r.area(), 8.0);
        assert_eq!(r.perimeter(), 12.0);
    }

    #[test]
    fn can_hold_is_strict() {
        let big = Rectangle::square(5.0);
        assert!(big.can_hold(&Rectangle::square(4.0)));
        assert!(!big.can_hold(&Rectangle::square(5.0)));
    }

    #[test]
    fn scale_doubles_sides() {
        let mut r = Rectangle {
            width: 1.0,
            height: 2.0,
        };
        r.scale(2.0);
        assert_eq!(
            r,
            Rectangle {
                width: 2.0,
                height: 4.0
            }
        );
    }
}
