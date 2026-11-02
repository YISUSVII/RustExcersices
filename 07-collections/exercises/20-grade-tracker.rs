use std::collections::HashMap;

#[derive(Debug, Default)]
struct GradeTracker {
    grades: HashMap<String, Vec<u32>>,
}

impl GradeTracker {
    fn new() -> GradeTracker {
        // TODO: implement
        todo!("Return an empty tracker (Default can help)")
    }

    fn add_grade(&mut self, student: &str, grade: u32) -> Result<(), String> {
        // TODO: implement
        let _ = (student, grade);
        todo!("Reject grades > 100, otherwise push via entry().or_default()")
    }

    fn average(&self, student: &str) -> Option<f64> {
        // TODO: implement
        let _ = student;
        todo!("None for unknown students, otherwise sum / count")
    }

    fn letter(average: f64) -> char {
        // TODO: implement
        let _ = average;
        todo!("A >= 90, B >= 80, C >= 70, D >= 60, else F")
    }

    fn report(&self) -> Vec<(String, f64, char)> {
        // TODO: implement
        todo!("One (name, average, letter) row per student, sorted by name")
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
