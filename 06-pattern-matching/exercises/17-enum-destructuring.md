# Exercise 17 — Enum Destructuring

- **Title:** Enum Destructuring Drills
- **Objective:** Pull data out of enum variants, structs and tuples with patterns.
- **Difficulty:** Intermediate
- **Concepts practiced:** struct/tuple variant patterns, `..`, match guards, `@` bindings, `|` alternatives

## Instructions

1. Implement `area(shape: &Shape) -> f64` for:
   - `Shape::Circle { radius }`
   - `Shape::Rectangle { width, height }`
   - `Shape::Triangle(a, b, c)` (use Heron's formula)
2. Implement `describe(msg: &Message) -> String` for:
   - `Message::Quit` → `"quit"`
   - `Message::Move { x: 0, y: 0 }` → `"stay"`
   - `Message::Move { x, y: 0 }` → `"move horizontally by <x>"`
   - `Message::Move { x: 0, y }` → `"move vertically by <y>"`
   - `Message::Move { x, y }` → `"move to (<x>, <y>)"`
   - `Message::Write(text)` → `"empty message"` when blank (guard), otherwise `"write: <text>"`
   - `Message::ChangeColor(r, g, b)` → `"grayscale <r>"` when `r == g && g == b`, otherwise `"color #RRGGBB"` (hex)
3. Implement `classify_age(age: u8) -> String` using `@` bindings with ranges: `0..=12` child, `13..=19` teen, others adult — e.g. `"teen (15)"`.

## Expected behavior

```text
circle area: 3.14
rect area: 6.00
triangle area: 6.00
stay
move horizontally by 5
write: hi
grayscale 128
color #FF8000
quit
teen (15)
```

## Optional hints

- Order matters: put more specific patterns (like `Move { x: 0, y: 0 }`) before general ones.
- `{:02X}` formats a byte as two uppercase hex digits.

## Bonus challenge

Add a `Shape::Square(f64)` variant and handle it with an `|` pattern together with `Rectangle`.
