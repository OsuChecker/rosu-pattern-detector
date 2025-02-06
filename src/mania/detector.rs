use crate::mania::structs::{BasePattern, ManiaMeasure, SecondaryPattern, TertiaryPattern};
use std::collections::{BTreeMap, HashMap};
use crate::mania::structs::TertiaryPattern::JT;

pub(crate) fn detect_primary_pattern_4k(note: &crate::mania::structs::Notes) -> BasePattern {
    let count = note.notes.iter().filter(|&&n| n).count();

    fn get_pattern(number: usize) -> BasePattern {
        match number {
            1 => BasePattern::Single,
            2 => BasePattern::Jump,
            3 => BasePattern::Hand,
            4 => BasePattern::Quad,
            _ => BasePattern::Chord,
        }
    }
    get_pattern(count)
}


pub(crate) fn analyze_patterns_by_measures_advanced(
    grouped_measures: &mut BTreeMap<i32, ManiaMeasure>,
) -> HashMap<String, f64>
{
    let mut pattern_counts: HashMap<String, f64> = HashMap::new();
    pattern_counts.insert("Jack".to_string(), 0.0);
    pattern_counts.insert("Jumpstream".to_string(), 0.0);
    pattern_counts.insert("Singlestream".to_string(), 0.0);
    pattern_counts.insert("Handstream".to_string(), 0.0);

    let measure_count = grouped_measures.len();
    let average_npm = grouped_measures
        .values()
        .map(|measure| measure.measure.npm as f64)
        .sum::<f64>()
        / measure_count.max(1) as f64;

    let amplification_power: f64 = 1.0;

    for measure in grouped_measures.values_mut() {
        let weight = if average_npm > 0.0 {
            (measure.measure.npm as f64 / average_npm).powf(amplification_power)
        } else {
            1.0
        };

        let mut has_jack = false;
        let mut has_jumpstream = false;
        let mut has_singlestream = false;
        let mut has_handstream = false;

        for (i, note) in measure.notes.iter().enumerate() {
            if i > 0 {
                let prev = &measure.notes[i - 1];
                if note.notes.iter().zip(prev.notes.iter()).any(|(n, p)| *n && *p) {
                    has_jack = true;
                }
            }

            match note.pattern {
                BasePattern::Hand => has_handstream = true,
                BasePattern::Jump => has_jumpstream = true,
                BasePattern::Single => has_singlestream = true,
                _ => {}
            }
        }

        if has_jack {
            *pattern_counts.get_mut("Jack").unwrap() += weight;
            measure.secondary_pattern = SecondaryPattern::Jack;
        } else if has_handstream {
            *pattern_counts.get_mut("Handstream").unwrap() += weight;
            measure.secondary_pattern = SecondaryPattern::Handstream;
        } else if has_jumpstream {
            *pattern_counts.get_mut("Jumpstream").unwrap() += weight;
            measure.secondary_pattern = SecondaryPattern::Jumpstream;
        } else if has_singlestream {
            *pattern_counts.get_mut("Singlestream").unwrap() += weight;
            measure.secondary_pattern = SecondaryPattern::Singlestream;
        }
    }

    pattern_counts
}


pub(crate) fn analyze_patterns_tertiary(
    grouped_measures: &mut BTreeMap<i32, ManiaMeasure>,
    key: i32,
) -> BTreeMap<TertiaryPattern, f64> {
    let mut map: BTreeMap<TertiaryPattern, f64> = BTreeMap::new();
    let measure_count = grouped_measures.len();
    let average_npm = grouped_measures
        .values()
        .map(|measure| measure.measure.npm as f64)
        .sum::<f64>()
        / measure_count.max(1) as f64;

    let amplification_power: f64 = 1.0;

    for measure in grouped_measures.values_mut() {
        let density_factor = if average_npm > 0.0 {
            (measure.measure.npm as f64 / average_npm) * 0.8
        } else {
            1.0
        };

        if measure.secondary_pattern == SecondaryPattern::Jack {
            let key = check_jack(measure);
            measure.tertiary_pattern = key.clone();
            *map.entry(key).or_insert(0.0) += density_factor;

        } else if measure.secondary_pattern == SecondaryPattern::Jumpstream {
            let key = check_js(measure);
            measure.tertiary_pattern = key.clone();
            *map.entry(key).or_insert(0.0) += density_factor;

        }
        else if measure.secondary_pattern==SecondaryPattern::Handstream{
            let key = check_hs(measure);
            *map.entry(key).or_insert(0.0) += density_factor;
        }

    }

    map
}

fn check_hs(measure: &mut ManiaMeasure) -> TertiaryPattern {
    let mut pattern_count: HashMap<BasePattern, usize> = HashMap::new();

    for note in measure.notes() {
        *pattern_count.entry(note.pattern.clone()).or_insert(0) += 1;
    }
    let single = *pattern_count.get(&BasePattern::Single).unwrap_or(&0);
    let jump = *pattern_count.get(&BasePattern::Jump).unwrap_or(&0);
    let hand = *pattern_count.get(&BasePattern::Hand).unwrap_or(&0);

    if jump==0{
        TertiaryPattern::LIGHT_HS
    } 
    else if jump>0 {
            TertiaryPattern::DENSE_HS
    }
    else {
        TertiaryPattern::HS
    }
}

fn check_jack(p0: &mut ManiaMeasure) -> TertiaryPattern {
    let mut pattern_count: HashMap<BasePattern, usize> = HashMap::new();

    for note in p0.notes() {
        *pattern_count.entry(note.pattern.clone()).or_insert(0) += 1;
    }
    let single = *pattern_count.get(&BasePattern::Single).unwrap_or(&0);
    let jump = *pattern_count.get(&BasePattern::Jump).unwrap_or(&0);
    let hand = *pattern_count.get(&BasePattern::Hand).unwrap_or(&0);
    let quad = *pattern_count.get(&BasePattern::Quad).unwrap_or(&0);
    let chord = *pattern_count.get(&BasePattern::Chord).unwrap_or(&0);

    if hand > jump+single
    {
        TertiaryPattern::DENSE_CHORDJACK
    }
    else if quad > 0 && jump + hand + quad > single
    {
        TertiaryPattern::CHORDJACK
    }
    else {
        check_jackspeed_or_chordstream(p0)
    }
}

fn check_jackspeed_or_chordstream(measure: &mut ManiaMeasure) -> TertiaryPattern {
    let mut jack_count = 0;
    for (i, note) in measure.notes.iter().enumerate() {
        if i > 0 {
            let prev = &measure.notes[i - 1];
            if note.notes.iter().zip(prev.notes.iter()).any(|(n, p)| *n && *p) {
                jack_count += 1;
            }
        }
    }
    if jack_count <= 1 && measure.tNotes()>6
    {
        TertiaryPattern::CHORDSTREAM

    } else {
        TertiaryPattern::SPEEDJACK
    }
}

fn check_js(measure: &mut ManiaMeasure) -> TertiaryPattern {

    if (has_two_consecutive_jumps(measure)) {
        return JT
    }
    let mut pattern_count: HashMap<BasePattern, usize> = HashMap::new();

    for note in measure.notes() {
        *pattern_count.entry(note.pattern.clone()).or_insert(0) += 1;
    }
    let single = *pattern_count.get(&BasePattern::Single).unwrap_or(&0);
    let jump = *pattern_count.get(&BasePattern::Jump).unwrap_or(&0);

    let mut vect_int = vec![0; measure.notes[0].notes.len()];

    for (i, note) in measure.notes.iter().enumerate() {
        for (i, &is_active) in note.notes.iter().enumerate() {
            if is_active {
                vect_int[i] += 1;
            }
        }
    }

    if let Some(&max_value) = vect_int.iter().max() {
        if max_value > 3 {
            return TertiaryPattern::ANCHOR_JS;
        } else if jump < single {
            TertiaryPattern::LIGHT_JS
        } else {
            TertiaryPattern::JS
        }
    } else {
        TertiaryPattern::JS
    }
}

fn has_two_consecutive_jumps(measure: &ManiaMeasure) -> bool {
    measure.notes
        .windows(2)
        .any(|pair| {
            pair.iter().all(|note| matches!(note.pattern, BasePattern::Jump))
        })
}
