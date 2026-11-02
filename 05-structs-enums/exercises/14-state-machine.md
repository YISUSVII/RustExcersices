# Exercise 14 — Order State Machine

- **Title:** Order State Machine
- **Objective:** Model an order lifecycle where only valid transitions are allowed.
- **Difficulty:** Intermediate
- **Concepts practiced:** enums with data, consuming `self`, `match` on tuples, `Result`

## Instructions

1. Use the provided enums:
   - `OrderState { Pending, Paid { amount: u64 }, Shipped { tracking: String }, Delivered, Cancelled }`
   - `Event { Pay(u64), Ship(String), Deliver, Cancel }`
2. Implement `OrderState::next(self, event: Event) -> Result<OrderState, String>` with these rules:
   - `Pending` + `Pay(amount)` → `Paid { amount }`
   - `Paid` + `Ship(tracking)` → `Shipped { tracking }`
   - `Shipped` + `Deliver` → `Delivered`
   - `Pending` or `Paid` + `Cancel` → `Cancelled`
   - anything else → `Err` describing the invalid transition
3. Implement `is_final(&self) -> bool` (`Delivered` and `Cancelled` are final).

## Expected behavior

```text
Pending -> Paid { amount: 4999 }
Paid { amount: 4999 } -> Shipped { tracking: "TRK-1" }
Shipped { tracking: "TRK-1" } -> Delivered
final: true
error: cannot apply Cancel to Delivered
```

## Optional hints

- Match on the tuple `(self, event)` so every state/event pair is handled in one place.
- Taking `self` by value means an old state can't be reused after a transition.

## Bonus challenge

Reject `Pay(0)` with an error.
