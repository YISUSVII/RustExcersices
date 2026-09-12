#[derive(Debug, PartialEq, Eq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(self) -> Self {
        // TODO: Green -> Yellow -> Red -> Green
        let _ = self;
        unimplemented!("Implement next")
    }

    fn label(&self) -> &'static str {
        let _ = self;
        unimplemented!("Implement label")
    }
}

fn main() {
    let mut light = TrafficLight::Green;
    for _ in 0..4 {
        println!("{}", light.label());
        light = light.next();
    }
}
