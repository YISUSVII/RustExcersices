const KM_TO_MILES: f64 = 0.621_371;

fn km_to_miles(km: f64) -> f64 {
    km * KM_TO_MILES
}

fn miles_to_km(miles: f64) -> f64 {
    miles / KM_TO_MILES
}

fn main() {
    println!("10 km = {:.3} miles", km_to_miles(10.0));
    println!("6.214 miles = {:.3} km", miles_to_km(6.214));
}
