use crate::mania::detector::{analyze_patterns_by_measures_advanced, analyze_patterns_tertiary};
use crate::mania::transform::{group_notes_by_measures, transform_hit_object_to_mania_notes};
use rosu_map::Beatmap;
use serde_json::json;

mod structs;
mod detector;
mod transform;

pub fn transformers(map: Beatmap) -> serde_json::Value {
    let timing_point = map.control_points.timing_points;

    let notes = transform_hit_object_to_mania_notes(map.hit_objects, map.circle_size as usize);
    let mut mesure = group_notes_by_measures(notes, timing_point);
    let Hash = analyze_patterns_by_measures_advanced(&mut mesure);
    let result_json = json!(Hash);
    let result_json_2 = json!(analyze_patterns_tertiary(&mut mesure, map.circle_size as i32));
    let combined = json!({
    "SecondaryPattern": result_json,
    "TertiaryPattern": result_json_2
        });
    combined

}