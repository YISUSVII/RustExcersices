#[derive(Debug, Clone, PartialEq)]
enum OrderState {
    Pending,
    Paid { amount: u64 },
    Shipped { tracking: String },
    Delivered,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq)]
enum Event {
    Pay(u64),
    Ship(String),
    Deliver,
    Cancel,
}

impl OrderState {
    fn next(self, event: Event) -> Result<OrderState, String> {
        match (self, event) {
            (OrderState::Pending, Event::Pay(0)) => {
                Err("payment amount must be positive".to_string())
            }
            (OrderState::Pending, Event::Pay(amount)) => Ok(OrderState::Paid { amount }),
            (OrderState::Paid { .. }, Event::Ship(tracking)) => {
                Ok(OrderState::Shipped { tracking })
            }
            (OrderState::Shipped { .. }, Event::Deliver) => Ok(OrderState::Delivered),
            (OrderState::Pending | OrderState::Paid { .. }, Event::Cancel) => {
                Ok(OrderState::Cancelled)
            }
            (state, event) => Err(format!("cannot apply {event:?} to {state:?}")),
        }
    }

    fn is_final(&self) -> bool {
        matches!(self, OrderState::Delivered | OrderState::Cancelled)
    }
}

fn main() {
    let events = [
        Event::Pay(4999),
        Event::Ship("TRK-1".to_string()),
        Event::Deliver,
    ];

    let mut state = OrderState::Pending;
    for event in events {
        let before = state.clone();
        match state.next(event) {
            Ok(next) => {
                println!("{before:?} -> {next:?}");
                state = next;
            }
            Err(e) => {
                println!("error: {e}");
                return;
            }
        }
    }
    println!("final: {}", state.is_final());

    if let Err(e) = state.next(Event::Cancel) {
        println!("error: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let s = OrderState::Pending
            .next(Event::Pay(10))
            .and_then(|s| s.next(Event::Ship("T".into())))
            .and_then(|s| s.next(Event::Deliver));
        assert_eq!(s, Ok(OrderState::Delivered));
    }

    #[test]
    fn cancel_only_before_shipping() {
        assert_eq!(
            OrderState::Pending.next(Event::Cancel),
            Ok(OrderState::Cancelled)
        );
        assert_eq!(
            OrderState::Paid { amount: 1 }.next(Event::Cancel),
            Ok(OrderState::Cancelled)
        );
        assert!(OrderState::Shipped {
            tracking: "T".into()
        }
        .next(Event::Cancel)
        .is_err());
    }

    #[test]
    fn invalid_transitions_fail() {
        assert!(OrderState::Pending.next(Event::Deliver).is_err());
        assert!(OrderState::Delivered.next(Event::Pay(5)).is_err());
    }

    #[test]
    fn final_states() {
        assert!(OrderState::Delivered.is_final());
        assert!(OrderState::Cancelled.is_final());
        assert!(!OrderState::Pending.is_final());
    }
}
