use crate::std::structs::{SecondaryPattern, StdMeasure};
use std::collections::{BTreeMap, HashMap};


fn calc_average_distance_speed_ratio(measure: &StdMeasure) -> f64 {
    if measure.notes.len() < 2 {
        return 0.0;
    }

    let total_distance: f64 = measure.notes
        .windows(2)
        .map(|pair| {
            let dx = pair[1].coordinates[0] as f64 - pair[0].coordinates[0] as f64;
            let dy = pair[1].coordinates[1] as f64 - pair[0].coordinates[1] as f64;
            (dx * dx + dy * dy).sqrt()
        })
        .sum();

    let average_distance = total_distance / (measure.notes.len() - 1) as f64;
    average_distance / (measure.measure.npm as f64)
}


pub fn detect_secondary_pattern(measures: &mut BTreeMap<i32, StdMeasure>) -> HashMap<SecondaryPattern, f64> {
    // On calcule d'abord les distances moyennes de chaque mesure dans un vecteur.
    let average_distances: Vec<f64> = measures
        .values()
        .map(|m| calc_average_distance_speed_ratio(m))
        .collect();


    let global_avg_distance = if !average_distances.is_empty() {
        average_distances.iter().sum::<f64>() / average_distances.len() as f64
    } else {
        1.0
    };

    let mut pattern_map: HashMap<SecondaryPattern, f64> = HashMap::new();
    pattern_map.insert(SecondaryPattern::STREAM, 0.0);
    pattern_map.insert(SecondaryPattern::JUMP, 0.0);


    for avg_dist in average_distances {
        let ratio = if global_avg_distance > 0.0 {
            avg_dist / global_avg_distance
        } else {
            1.0
        };


        if avg_dist < global_avg_distance {
            *pattern_map.get_mut(&SecondaryPattern::STREAM).unwrap() += 1f64;
        } else {
            *pattern_map.get_mut(&SecondaryPattern::JUMP).unwrap() += 1f64;
        }
    }

    pattern_map
}
