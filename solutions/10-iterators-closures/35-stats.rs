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
    if data.is_empty() {
        return None;
    }
    Some(data.iter().sum::<f64>() / data.len() as f64)
}

fn sorted_copy(data: &[f64]) -> Vec<f64> {
    let mut sorted = data.to_vec();
    sorted.sort_by(f64::total_cmp);
    sorted
}

fn median(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    let sorted = sorted_copy(data);
    let mid = sorted.len() / 2;
    Some(if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    })
}

fn mode(data: &[i64]) -> Option<i64> {
    let counts = data.iter().fold(HashMap::new(), |mut map, &x| {
        *map.entry(x).or_insert(0usize) += 1;
        map
    });
    counts
        .into_iter()
        // Higher count wins; on equal counts the smaller value wins.
        .max_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0)))
        .map(|(value, _)| value)
}

fn variance(data: &[f64]) -> Option<f64> {
    let m = mean(data)?;
    let sum_sq = data.iter().map(|x| (x - m).powi(2)).sum::<f64>();
    Some(sum_sq / data.len() as f64)
}

fn std_dev(data: &[f64]) -> Option<f64> {
    variance(data).map(f64::sqrt)
}

fn min_max(data: &[f64]) -> Option<(f64, f64)> {
    if data.is_empty() {
        return None;
    }
    Some(
        data.iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &x| {
                (lo.min(x), hi.max(x))
            }),
    )
}

fn summarize(data: &[f64]) -> Option<Summary> {
    let (min, max) = min_max(data)?;
    Some(Summary {
        count: data.len(),
        mean: mean(data)?,
        median: median(data)?,
        min,
        max,
        std_dev: std_dev(data)?,
    })
}

fn percentile(data: &[f64], p: f64) -> Option<f64> {
    if data.is_empty() || !(0.0..=100.0).contains(&p) {
        return None;
    }
    let sorted = sorted_copy(data);
    let rank = p / 100.0 * (sorted.len() - 1) as f64;
    let (lo, hi) = (rank.floor() as usize, rank.ceil() as usize);
    let frac = rank - lo as f64;
    Some(sorted[lo] + (sorted[hi] - sorted[lo]) * frac)
}

fn parse_data(input: &str) -> Result<Vec<f64>, String> {
    input
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| match s.parse::<f64>() {
            Ok(n) if !n.is_nan() => Ok(n),
            _ => Err(format!("invalid number: '{s}'")),
        })
        .collect()
}

fn main() {
    let data = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    println!("data: {data:?}");
    println!("mean: {:?}", mean(&data));
    println!("median: {:?}", median(&data));
    println!("variance: {:?}", variance(&data));
    println!("std_dev: {:?}", std_dev(&data));
    println!("min_max: {:?}", min_max(&data));
    println!("p90: {:?}", percentile(&data, 90.0));
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
        assert_eq!(percentile(&[1.0, 2.0, 3.0], 50.0), Some(2.0));
        assert_eq!(percentile(&data, 101.0), None);
    }

    #[test]
    fn parsing_rejects_bad_input() {
        assert_eq!(parse_data("1, 2.5 ,3"), Ok(vec![1.0, 2.5, 3.0]));
        assert_eq!(parse_data("1, 2.5,x"), Err("invalid number: 'x'".into()));
        assert!(parse_data("NaN").is_err());
    }
}
