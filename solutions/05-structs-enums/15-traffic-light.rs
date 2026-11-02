use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }

    fn duration_secs(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 25,
        }
    }

    fn can_go(&self) -> bool {
        matches!(self, TrafficLight::Green)
    }
}

impl fmt::Display for TrafficLight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            TrafficLight::Red => "RED",
            TrafficLight::Yellow => "YELLOW",
            TrafficLight::Green => "GREEN",
        };
        write!(f, "{label}")
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
