use crate::mania::detector::analyze_patterns_by_measures_advanced;
use crate::mania::models::base::ManiaMeasure;
use crate::mania::models::base::NotesStruct;
use crate::mania::models::pattern::Pattern;
use crate::mania::transform::{group_notes_by_measures, transform_hit_object_to_mania_notes};

use rosu_map::section::timing_points::TimingPoint;
use rosu_map::Beatmap;

use std::collections::{BTreeMap, HashMap};

pub mod detector;
pub mod transform;
pub mod models;

pub fn transformers(map: Beatmap) -> HashMap<Pattern, f64> {
    let timing_point: Vec<TimingPoint> = map.control_points.timing_points;
    let circle_size: i32 = map.circle_size as i32;

    let notes: Vec<NotesStruct> = transform_hit_object_to_mania_notes(map.hit_objects, circle_size);
    let mut measures: BTreeMap<i32, ManiaMeasure> = group_notes_by_measures(notes, timing_point);

    analyze_patterns_by_measures_advanced(&mut measures)
}
