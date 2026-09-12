use std::collections::HashMap;

macro_rules! hashmap {
    ( $( $k:expr => $v:expr ),* $(,)? ) => {{
        let mut m = HashMap::new();
        $( m.insert($k, $v); )*
        m
    }};
}

fn main() {
    let m: HashMap<&str, i32> = hashmap!{"a" => 1, "b" => 2};
    println!("{m:?}");
}
