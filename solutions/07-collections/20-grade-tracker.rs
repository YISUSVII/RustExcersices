use std::collections::HashMap;

#[derive(Debug, Default)]
struct GradeTracker {
    grades: HashMap<String, Vec<u32>>,
}

impl GradeTracker {
    fn new() -> GradeTracker {
        GradeTracker::default()
    }

    fn add_grade(&mut self, student: &str, grade: u32) -> Result<(), String> {
        if grade > 100 {
            return Err(format!("grade {grade} is out of range (0-100)"));
        }
        self.grades
            .entry(student.to_string())
            .or_default()
            .push(grade);
        Ok(())
    }

    fn average(&self, student: &str) -> Option<f64> {
        let grades = self.grades.get(student)?;
        if grades.is_empty() {
            return None;
        }
        let sum: u32 = grades.iter().sum();
        Some(sum as f64 / grades.len() as f64)
    }

    fn letter(average: f64) -> char {
        match average {
            a if a >= 90.0 => 'A',
            a if a >= 80.0 => 'B',
            a if a >= 70.0 => 'C',
            a if a >= 60.0 => 'D',
            _ => 'F',
        }
    }

    fn report(&self) -> Vec<(String, f64, char)> {
        let mut rows: Vec<(String, f64, char)> = self
            .grades
            .keys()
            .filter_map(|name| {
                self.average(name)
                    .map(|avg| (name.clone(), avg, GradeTracker::letter(avg)))
            })
            .collect();
        rows.sort_by(|a, b| a.0.cmp(&b.0));
        rows
    }

    fn best_student(&self) -> Option<&str> {
        self.grades
            .keys()
            .filter_map(|name| self.average(name).map(|avg| (name.as_str(), avg)))
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(name, _)| name)
    }
}

fn main() {
    let mut tracker = GradeTracker::new();
    let entries = [
        ("ana", 95),
        ("ben", 70),
        ("ana", 87),
        ("cleo", 55),
        ("ben", 79),
        ("ben", 120),
    ];
    for (student, grade) in entries {
        if let Err(e) = tracker.add_grade(student, grade) {
            println!("rejected: {e}");
        }
    }

    for (name, avg, letter) in tracker.report() {
        println!("{name}: {avg:.1} ({letter})");
    }
    println!("unknown: {:?}", tracker.average("zed"));
    println!("best: {:?}", tracker.best_student());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_out_of_range() {
        let mut t = GradeTracker::new();
        assert!(t.add_grade("a", 101).is_err());
        assert_eq!(t.average("a"), None);
    }

    #[test]
    fn averages() {
        let mut t = GradeTracker::new();
        t.add_grade("a", 80).unwrap();
        t.add_grade("a", 90).unwrap();
        assert_eq!(t.average("a"), Some(85.0));
    }

    #[test]
    fn letters() {
        assert_eq!(GradeTracker::letter(90.0), 'A');
        assert_eq!(GradeTracker::letter(89.9), 'B');
        assert_eq!(GradeTracker::letter(60.0), 'D');
        assert_eq!(GradeTracker::letter(10.0), 'F');
    }

    #[test]
    fn report_is_sorted() {
        let mut t = GradeTracker::new();
        t.add_grade("zoe", 50).unwrap();
        t.add_grade("amy", 100).unwrap();
        let names: Vec<String> = t.report().into_iter().map(|r| r.0).collect();
        assert_eq!(names, vec!["amy", "zoe"]);
    }
}
