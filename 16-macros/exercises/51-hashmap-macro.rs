use std::collections::HashMap;

// TODO: implement hashmap! with "key" => value pairs
macro_rules! hashmap {
    ( $( $k:expr => $v:expr ),* $(,)? ) => {{
        let mut m = HashMap::new();
        // TODO: insert each pair
        let _ = ( $( $k, $v, )* );
        m
    }};
}

fn main() {
    let m: HashMap<&str, i32> = hashmap!{"a" => 1, "b" => 2};
    println!("{m:?}");
}
