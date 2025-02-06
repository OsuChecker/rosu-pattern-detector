use crate::std::structs::{BasePattern, Notes, SecondaryPattern, StdMeasure};
use rosu_map::section::hit_objects::{HitObject, HitObjectKind};
use rosu_map::section::timing_points::TimingPoint;
use std::collections::BTreeMap;

pub(crate) fn transform_hit_object_to_std_notes(ho: Vec<HitObject>) -> Vec<Notes>
{
    let mut notes_list = ho.iter()
        .filter_map(|hit_object| {
            match &hit_object.kind {
                HitObjectKind::Circle(circle) => Some(Notes {
                    timestamp: hit_object.start_time as i32,
                    pattern: BasePattern::SINGLE,
                    coordinates: vec![circle.pos.x, circle.pos.y],
                }),
                HitObjectKind::Slider(slider) => Some(Notes {
                    timestamp: hit_object.start_time as i32,
                    pattern: BasePattern::SLIDERS,
                    coordinates: vec![slider.pos.x, slider.pos.y],
                }),
                _ => None,
            }
        })
        .collect::<Vec<_>>();
    notes_list.sort_by_key(|note| note.timestamp);
    notes_list
}


pub(crate) fn group_notes_by_measures(
    notes: Vec<Notes>,
    timing_points: Vec<TimingPoint>,
) -> BTreeMap<i32, StdMeasure> {
    let mut measures = BTreeMap::new();

    for note in notes {
        let timing_point = timing_points
            .iter()
            .rev()
            .find(|tp| note.timestamp >= tp.time as i32)
            .unwrap_or_else(|| timing_points.first().unwrap());

        let beat_len = timing_point.beat_len as f32;
        let start_time = timing_point.time as i32;

        let measure_idx = ((note.timestamp - start_time) as f32 / beat_len).floor() as i32;
        let measure_start_time = start_time + (measure_idx as f32 * beat_len) as i32;

        let measure_entry = measures.entry(measure_start_time).or_insert_with(|| StdMeasure {
            measure: crate::structs::CommonMeasure {
                start_time: measure_start_time,
                npm: 0,
            },
            notes: Vec::new(),
            secondary_pattern: SecondaryPattern::None,
        });


        measure_entry.notes.push(note.clone());
        measure_entry.measure.npm += 1;
    }

    measures
}