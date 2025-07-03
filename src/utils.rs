use crate::mania::models::pattern::Pattern;
use std::collections::HashMap;

pub fn max_values(patterns: &HashMap<Pattern, f64>) -> Vec<(Pattern, f64)> {
    let mut result = Vec::new();
    if patterns.is_empty() {
        return result;
    }

    let mut sorted: Vec<_> = patterns.iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let max_value = sorted[0].1;

    for (pattern, value) in sorted {
        if value < max_value * 0.5 {
            break;
        }
        result.push((pattern, value));
    }

    result
}


pub fn sum_by_secondary_type(patterns: &HashMap<Pattern, f64>) -> HashMap<Pattern, f64> {
    let mut sums: HashMap<Pattern, f64> = HashMap::new();
    for (pattern, &value) in patterns {
        *sums.entry(pattern.to_all()).or_insert(0.0) += value;
    }
    sums
}

pub fn max_by_secondary_type(patterns: &HashMap<Pattern, f64>) -> Vec<(Pattern, f64)> {
    let sums = sum_by_secondary_type(patterns);

    let mut sorted: Vec<_> = sums.into_iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    if sorted.is_empty() {
        return Vec::new();
    }

    let max_value = sorted[0].1;

    sorted.into_iter()
        .take_while(|(_, value)| *value >= max_value * 0.5)
        .collect()
}
