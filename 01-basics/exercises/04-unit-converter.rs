const KM_TO_MILES: f64 = 0.621_371;

fn km_to_miles(km: f64) -> f64 {
    // TODO: implement using KM_TO_MILES
    let _ = (km, KM_TO_MILES);
    unimplemented!("Implement km_to_miles")
}

fn miles_to_km(miles: f64) -> f64 {
    // TODO: implement using KM_TO_MILES
    let _ = (miles, KM_TO_MILES);
    unimplemented!("Implement miles_to_km")
}

fn main() {
    println!("10 km = {:.3} miles", km_to_miles(10.0));
    println!("6.214 miles = {:.3} km", miles_to_km(6.214));
}
