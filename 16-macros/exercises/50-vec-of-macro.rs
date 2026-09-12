// TODO: complete the macro so pushes actually happen for the list form.
macro_rules! vec_of {
    () => {
        Vec::new()
    };
    ( $( $x:expr ),+ $(,)? ) => {{
        let mut v = Vec::new();
        // TODO: use macro repetition to push each $x
        // $( v.push($x); )+
        let _ = ( $( &$x, )+ );
        let _ = &mut v;
        v
    }};
}

fn main() {
    let v: Vec<i32> = vec_of![];
    let w: Vec<i32> = vec_of![1, 2, 3];
    println!("{v:?} {w:?}");
}
