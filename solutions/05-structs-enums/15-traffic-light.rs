#[derive(Debug, PartialEq, Eq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(self) -> Self {
        match self {
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Red => TrafficLight::Green,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            TrafficLight::Red => "red",
            TrafficLight::Yellow => "yellow",
            TrafficLight::Green => "green",
        }
    }
}

fn main() {
    let mut light = TrafficLight::Green;
    for _ in 0..4 {
        println!("{}", light.label());
        light = light.next();
    }
}
