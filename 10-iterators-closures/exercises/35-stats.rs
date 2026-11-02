use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Summary {
    count: usize,
    mean: f64,
    median: f64,
    min: f64,
    max: f64,
    std_dev: f64,
}

fn mean(data: &[f64]) -> Option<f64> {
    // TODO: implement
    let _ = data;
    todo!("None for empty input, otherwise sum / len")
}

fn median(data: &[f64]) -> Option<f64> {
    // TODO: implement (sort a copy with f64::total_cmp)
    let _ = data;
    todo!("Middle value, or the average of the two middle values")
}

fn mode(data: &[i64]) -> Option<i64> {
    // TODO: implement
    let _: HashMap<i64, usize> = HashMap::new();
    let _ = data;
    todo!("Most frequent value; smallest value wins ties")
}

fn variance(data: &[f64]) -> Option<f64> {
    // TODO: implement population variance
    let _ = data;
    todo!("Mean of squared differences from the mean")
}

fn std_dev(data: &[f64]) -> Option<f64> {
    // TODO: implement
    let _ = data;
    todo!("Square root of the variance")
}

fn min_max(data: &[f64]) -> Option<(f64, f64)> {
    // TODO: implement with a single fold
    let _ = data;
    todo!("Fold starting from (INFINITY, NEG_INFINITY)")
}

fn summarize(data: &[f64]) -> Option<Summary> {
    // TODO: implement by reusing the functions above
    let _ = data;
    todo!("Build a Summary, or None for empty input")
}

fn parse_data(input: &str) -> Result<Vec<f64>, String> {
    // TODO: implement
    let _ = input;
    todo!("Split on ',', trim, parse, reject NaN")
}

fn main() {
    let data = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    println!("data: {data:?}");
    println!("mean: {:?}", mean(&data));
    println!("median: {:?}", median(&data));
    println!("variance: {:?}", variance(&data));
    println!("std_dev: {:?}", std_dev(&data));
    println!("min_max: {:?}", min_max(&data));
    println!("mode([1,2,2,3,3]): {:?}", mode(&[1, 2, 2, 3, 3]));
    println!("mean([]): {:?}", mean(&[]));

    match parse_data("3.5, 1, 2, 10") {
        Ok(values) => println!("summary: {:?}", summarize(&values)),
        Err(e) => eprintln!("error: {e}"),
    }
    println!("bad input: {:?}", parse_data("1, 2.5,x"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_and_median() {
        assert_eq!(mean(&[1.0, 2.0, 3.0, 4.0]), Some(2.5));
        assert_eq!(mean(&[]), None);
        assert_eq!(median(&[3.0, 1.0, 2.0]), Some(2.0));
        assert_eq!(median(&[4.0, 1.0, 3.0, 2.0]), Some(2.5));
    }

    #[test]
    fn mode_prefers_smallest_on_tie() {
        assert_eq!(mode(&[1, 2, 2, 3, 3]), Some(2));
        assert_eq!(mode(&[7]), Some(7));
        assert_eq!(mode(&[]), None);
    }

    #[test]
    fn spread_and_summary() {
        let data = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert_eq!(variance(&data), Some(4.0));
        assert_eq!(std_dev(&data), Some(2.0));
        let s = summarize(&data).expect("non-empty data");
        assert_eq!((s.count, s.min, s.max, s.mean), (8, 2.0, 9.0, 5.0));
    }

    #[test]
    fn parsing_rejects_bad_input() {
        assert_eq!(parse_data("1, 2.5 ,3"), Ok(vec![1.0, 2.5, 3.0]));
        assert_eq!(parse_data("1, 2.5,x"), Err("invalid number: 'x'".into()));
        assert!(parse_data("NaN").is_err());
    }
}
