use rosu_map;
use rosu_map::section::hit_objects::{HitObject, HitObjectKind};
use rosu_map::section::timing_points::TimingPoint;
use std::collections::BTreeMap;

pub(crate) fn transform_hit_object_to_mania_notes(
    ho: Vec<HitObject>,
    num_keys: usize,
) -> Vec<crate::mania::structs::Notes> {
    let positions = match num_keys {
        4 => vec![64.0, 192.0, 320.0, 448.0],
        7 => vec![36.0, 109.0, 182.0, 256.0, 329.0, 402.0, 475.0],
        _ => return Vec::new(),
    };
    let mut grouped: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
    for hit_object in ho {
        let (pos_x, timestamp) = match &hit_object.kind {
            HitObjectKind::Circle(circle) => (circle.pos.x, hit_object.start_time as i32),
            HitObjectKind::Slider(slider) => (slider.pos.x, hit_object.start_time as i32),
            _ => continue,
        };
        if let Some(key_index) = positions.iter().position(|&x| x == pos_x) {
            grouped.entry(timestamp).or_default().push(key_index);
        }
    }
    let mut notes_vec = Vec::with_capacity(grouped.len());
    for (timestamp, indices) in grouped {
        let mut keys = vec![false; num_keys];
        for &key_index in &indices {
            keys[key_index] = true;
        }
        let temporary_note = crate::mania::structs::Notes {
            timestamp,
            notes: keys.clone(),
            pattern: crate::mania::structs::BasePattern::None,
        };
        notes_vec.push(crate::mania::structs::Notes {
            timestamp,
            notes: keys,
            pattern: crate::mania::detector::detect_primary_pattern_4k(&temporary_note),
        });
    }
    notes_vec
}


pub(crate) fn group_notes_by_measures(
    notes: Vec<crate::mania::structs::Notes>,
    timing_points: Vec<TimingPoint>,
) -> BTreeMap<i32, crate::mania::structs::ManiaMeasure> {
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

        let measure_entry = measures.entry(measure_start_time).or_insert_with(|| crate::mania::structs::ManiaMeasure {
            measure: crate::structs::CommonMeasure {
                start_time: measure_start_time,
                npm: 0,
            },
            notes: Vec::new(),
            secondary_pattern: crate::mania::structs::SecondaryPattern::None,
            tertiary_pattern: crate::mania::structs::TertiaryPattern::None,
        });


        measure_entry.notes.push(note.clone());
        measure_entry.measure.npm += note.notes.iter().filter(|&&n| n).count() as i32;
    }

    measures
}