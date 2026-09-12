macro_rules! vec_of {
    () => {
        Vec::new()
    };
    ( $( $x:expr ),+ $(,)? ) => {{
        let mut v = Vec::new();
        $( v.push($x); )+
        v
    }};
}

fn main() {
    let v: Vec<i32> = vec_of![];
    let w: Vec<i32> = vec_of![1, 2, 3];
    println!("{v:?} {w:?}");
}
