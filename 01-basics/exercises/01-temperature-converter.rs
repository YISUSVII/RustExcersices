fn c_to_f(celsius: f64) -> f64 {
    // TODO: implement
    let _ = celsius;
    unimplemented!("Implement c_to_f using F = C * 9.0 / 5.0 + 32.0")
}

fn f_to_c(fahrenheit: f64) -> f64 {
    // TODO: implement
    let _ = fahrenheit;
    unimplemented!("Implement f_to_c using C = (F - 32.0) * 5.0 / 9.0")
}

fn main() {
    println!("0°C => {}°F", c_to_f(0.0));
    println!("212°F => {}°C", f_to_c(212.0));
}
