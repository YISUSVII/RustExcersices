#[derive(Debug)]
struct Order {
    weight_kg: Option<u32>,
    country: &'static str,
    express: bool,
}

// Original nested version, kept to prove both implementations agree.
fn shipping_cost_legacy(order: &Order) -> Result<u32, &'static str> {
    if let Some(w) = order.weight_kg {
        if w == 0 {
            Err("empty package")
        } else if order.country == "US" {
            if order.express {
                if w > 20 {
                    Err("too heavy for express")
                } else {
                    Ok(2000)
                }
            } else {
                Ok(500 + w * 100)
            }
        } else if order.express {
            Err("no international express")
        } else if w > 30 {
            Err("too heavy for international")
        } else {
            Ok(1500 + w * 200)
        }
    } else {
        Err("missing weight")
    }
}

fn shipping_cost(order: &Order) -> Result<u32, &'static str> {
    // TODO: implement
    let _ = order;
    todo!("Use let-else for the weight, then one match on (country, express, weight) with guards")
}

fn parse_point(input: &str) -> Option<(i32, i32)> {
    // TODO: implement
    let _ = input;
    todo!("split_once(',') and parse both halves, using let-else for each step")
}

fn main() {
    let orders = [
        Order {
            weight_kg: Some(3),
            country: "US",
            express: false,
        },
        Order {
            weight_kg: Some(5),
            country: "US",
            express: true,
        },
        Order {
            weight_kg: Some(2),
            country: "MX",
            express: true,
        },
        Order {
            weight_kg: None,
            country: "US",
            express: false,
        },
    ];
    for order in &orders {
        println!("{order:?} => {:?}", shipping_cost(order));
    }
    let agrees = orders
        .iter()
        .all(|o| shipping_cost(o) == shipping_cost_legacy(o));
    println!("matches legacy: {agrees}");
    println!("parse_point(\"3,-4\") = {:?}", parse_point("3,-4"));
    println!("parse_point(\"oops\") = {:?}", parse_point("oops"));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_orders() -> Vec<Order> {
        let mut orders = Vec::new();
        for weight_kg in [
            None,
            Some(0),
            Some(1),
            Some(20),
            Some(21),
            Some(30),
            Some(31),
        ] {
            for country in ["US", "MX", "DE"] {
                for express in [false, true] {
                    orders.push(Order {
                        weight_kg,
                        country,
                        express,
                    });
                }
            }
        }
        orders
    }

    #[test]
    fn refactor_matches_legacy() {
        for order in all_orders() {
            assert_eq!(
                shipping_cost(&order),
                shipping_cost_legacy(&order),
                "{order:?}"
            );
        }
    }

    #[test]
    fn specific_costs() {
        let o = Order {
            weight_kg: Some(21),
            country: "US",
            express: true,
        };
        assert_eq!(shipping_cost(&o), Err("too heavy for express"));
        let o = Order {
            weight_kg: Some(10),
            country: "DE",
            express: false,
        };
        assert_eq!(shipping_cost(&o), Ok(3500));
    }

    #[test]
    fn parse_points() {
        assert_eq!(parse_point("3,-4"), Some((3, -4)));
        assert_eq!(parse_point(" 1 , 2 "), Some((1, 2)));
        assert_eq!(parse_point("1;2"), None);
        assert_eq!(parse_point("a,2"), None);
    }
}
