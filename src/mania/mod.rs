use crate::mania::models::base::NotesStruct;
use crate::mania::transform::{group_notes_by_measures, transform_hit_object_to_mania_notes};
use crate::mania::detector::HitObjects;
use crate::mania::detector::analyze_patterns;

use rosu_map::section::timing_points::TimingPoint;
use rosu_map::Beatmap;

pub mod detector;
pub mod transform;
pub mod models;

pub fn transform_to_hit_objects(map: Beatmap) -> HitObjects {
    let timing_point: Vec<TimingPoint> = map.control_points.timing_points;
    let circle_size: i32 = map.circle_size as i32;

    let notes: Vec<NotesStruct> = transform_hit_object_to_mania_notes(map.hit_objects, circle_size);
    let mut measures: HitObjects = group_notes_by_measures(notes, timing_point);
    analyze_patterns(&mut measures);

    measures
}



