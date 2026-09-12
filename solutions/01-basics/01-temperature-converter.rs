fn c_to_f(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn f_to_c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    println!("0°C => {:.2}°F", c_to_f(0.0));
    println!("212°F => {:.2}°C", f_to_c(212.0));
}
