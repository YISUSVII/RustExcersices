use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(self) -> TrafficLight {
        // TODO: implement
        todo!("Red -> Green -> Yellow -> Red")
    }

    fn duration_secs(&self) -> u32 {
        // TODO: implement
        todo!("Red 30, Yellow 5, Green 25")
    }

    fn can_go(&self) -> bool {
        // TODO: implement
        todo!("Only Green allows going")
    }
}

impl fmt::Display for TrafficLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: implement
        let _ = f;
        todo!("write!(f, ...) with RED / YELLOW / GREEN")
    }
}

fn main() {
    let mut light = TrafficLight::Red;
    let mut total = 0;
    for _ in 0..3 {
        println!(
            "{light} for {}s (go: {})",
            light.duration_secs(),
            light.can_go()
        );
        total += light.duration_secs();
        light = light.next();
    }
    println!("cycle: {total}s");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle_returns_to_start() {
        let start = TrafficLight::Red;
        assert_eq!(start.next().next().next(), start);
    }

    #[test]
    fn only_green_can_go() {
        assert!(TrafficLight::Green.can_go());
        assert!(!TrafficLight::Red.can_go());
        assert!(!TrafficLight::Yellow.can_go());
    }

    #[test]
    fn display_and_durations() {
        assert_eq!(TrafficLight::Yellow.to_string(), "YELLOW");
        let total: u32 = [TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green]
            .iter()
            .map(TrafficLight::duration_secs)
            .sum();
        assert_eq!(total, 60);
    }
}
