use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use rosu_map::Beatmap;
use rosu_map::section::timing_points::{TimingPoint, TimingPoints};
use serde_json::json;
use crate::mania::detector::{analyze_patterns_by_measures_advanced, analyze_patterns_tertiary};
use crate::mania::structs::{ManiaMeasure, Notes, SecondaryPattern, TertiaryPattern};
use crate::mania::transform::{group_notes_by_measures, transform_hit_object_to_mania_notes};
mod structs;
mod detector;
mod transform;

pub fn transformers(map: Beatmap) -> (HashMap<SecondaryPattern, f64> ,HashMap<TertiaryPattern, f64> ) {
    let timing_point : Vec<TimingPoint> = map.control_points.timing_points;
    let circle_size : i32 = map.circle_size as i32;

    let notes : Vec<Notes> = transform_hit_object_to_mania_notes(map.hit_objects, circle_size);
    let mut measures: BTreeMap<i32,ManiaMeasure> = group_notes_by_measures(notes, timing_point);

    (    analyze_patterns_by_measures_advanced(&mut measures), analyze_patterns_tertiary(&mut measures, circle_size))
}
