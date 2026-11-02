# Exercise 20 — Grade Tracker

- **Title:** Grade Tracker
- **Objective:** Store grades per student and compute averages and letter grades.
- **Difficulty:** Intermediate
- **Concepts practiced:** `HashMap<String, Vec<u32>>`, `entry().or_default()`, `Option`, iterator `sum`/`map`, sorting

## Instructions

Implement `GradeTracker` (a wrapper around `HashMap<String, Vec<u32>>`):

1. `GradeTracker::new() -> GradeTracker`
2. `add_grade(&mut self, student: &str, grade: u32) -> Result<(), String>` — reject grades above `100`.
3. `average(&self, student: &str) -> Option<f64>` — `None` for unknown students.
4. `letter(average: f64) -> char` — `A` ≥ 90, `B` ≥ 80, `C` ≥ 70, `D` ≥ 60, otherwise `F`.
5. `report(&self) -> Vec<(String, f64, char)>` — one row per student, sorted by name.

## Expected behavior

```text
rejected: grade 120 is out of range (0-100)
ana: 91.0 (A)
ben: 74.5 (C)
cleo: 55.0 (F)
unknown: None
```

## Optional hints

- `self.grades.entry(student.to_string()).or_default().push(grade);`
- Average = `sum as f64 / len as f64`.
- `match` with guards works nicely for `letter`: `a if a >= 90.0 => 'A'`.

## Bonus challenge

Add `best_student(&self) -> Option<&str>`.
