use std::collections::{HashMap, HashSet};

/// `hashmap!{ "a" => 1, "b" => 2 }` builds a `HashMap`.
macro_rules! hashmap {
    () => {
        HashMap::new()
    };
    ($($key:expr => $value:expr),+ $(,)?) => {{
        // TODO: implement — create a mutable map, insert every pair with $(...)+, return it
        $(let _ = (&$key, &$value);)+
        todo!("Use a repetition: $(map.insert($key, $value);)+")
    }};
}

/// `hashset![1, 2, 3]` builds a `HashSet`.
macro_rules! hashset {
    ($($item:expr),* $(,)?) => {{
        // TODO: implement — like hashmap!, but with HashSet::insert
        $(let _ = &$item;)*
        todo!("Insert every item into a HashSet")
    }};
}

/// Recursive: `max!(a)`, `max!(a, b)`, `max!(a, b, c, ...)`.
macro_rules! max {
    ($x:expr $(,)?) => {
        $x
    };
    ($x:expr, $($rest:expr),+ $(,)?) => {{
        // TODO: implement — compare `first` with max!($($rest),+) (recursion!)
        let first = $x;
        $(let _ = $rest;)+
        first
    }};
}

macro_rules! min {
    ($x:expr $(,)?) => {
        $x
    };
    ($x:expr, $($rest:expr),+ $(,)?) => {{
        // TODO: implement — mirror max!
        let first = $x;
        $(let _ = $rest;)+
        first
    }};
}

/// `square!(expr)` should evaluate `expr` exactly once.
macro_rules! square {
    ($x:expr) => {
        // TODO: fix — this naive version evaluates `$x` twice. Bind it to a variable first.
        $x * $x
    };
}

fn main() {
    let ages: HashMap<&str, i32> = hashmap! {
        "Ana" => 31,
        "Ben" => 25,
        "Cy" => 40,
    };
    let mut names: Vec<_> = ages.keys().collect();
    names.sort();
    println!("people: {names:?}");
    println!("Ben is {}", ages["Ben"]);

    let empty: HashMap<String, i32> = hashmap! {};
    println!("empty map len = {}", empty.len());

    let tags: HashSet<&str> = hashset!["rust", "macros", "rust"];
    println!("unique tags = {}", tags.len());

    println!("max!(3, 9, 2, 7) = {}", max!(3, 9, 2, 7));
    println!("min!(3.5, -1.0, 2.0) = {}", min!(3.5, -1.0, 2.0));
    println!("max!(\"pear\", \"apple\") = {}", max!("pear", "apple"));

    let mut calls = 0;
    let mut next = || {
        calls += 1;
        4
    };
    println!("square!(next()) = {}", square!(next()));
    println!("next() was called {calls} time(s)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashmap_macro() {
        let m: HashMap<i32, &str> = hashmap! { 1 => "one", 2 => "two", };
        assert_eq!(m.len(), 2);
        assert_eq!(m[&2], "two");
        let e: HashMap<u8, u8> = hashmap! {};
        assert!(e.is_empty());
    }

    #[test]
    fn hashset_macro_dedups() {
        let s: HashSet<i32> = hashset![1, 2, 2, 3, 3, 3];
        assert_eq!(s.len(), 3);
        let e: HashSet<i32> = hashset![];
        assert!(e.is_empty());
    }

    #[test]
    fn recursive_max_min() {
        assert_eq!(max!(5), 5);
        assert_eq!(max!(1, 8, 3, 8, -2), 8);
        assert_eq!(min!(4, 2, 9, 1, 7), 1);
        assert_eq!(max!("b", "c", "a"), "c");
    }

    #[test]
    fn square_evaluates_once() {
        let mut n = 0;
        let r = square!({
            n += 1;
            n + 2
        });
        assert_eq!((r, n), (9, 1));
    }
}
