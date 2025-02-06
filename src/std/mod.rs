use crate::std::detector::detect_secondary_pattern;
use crate::std::transform::{group_notes_by_measures, transform_hit_object_to_std_notes};
use rosu_map::Beatmap;
use serde_json::json;

mod transform;
mod detector;
mod structs;


pub fn transformers(map: Beatmap) -> serde_json::Value {
    let timing_point = map.control_points.timing_points;

    let notes = transform_hit_object_to_std_notes(map.hit_objects);
    let mut mesure = group_notes_by_measures(notes, timing_point);
    let Hash = detect_secondary_pattern(&mut mesure);
    let result_json = json!(Hash);
    result_json
}