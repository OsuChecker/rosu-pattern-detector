use crate::mania::structs::{BasePattern, HandstreamPattern, JackPattern, JumpstreamPattern, ManiaMeasure, Notes, SecondaryPattern, SinglestreamPattern};
use std::collections::{BTreeMap, HashMap};

pub(crate) fn detect_primary_pattern_4k(note: &Notes) -> BasePattern {
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
) -> HashMap<SecondaryPattern, f64> {

    let mut pattern_counts = HashMap::with_capacity(4);

    let measure_count = grouped_measures.len();
    let average_npm = grouped_measures
        .values()
        .map(|measure| measure.measure.npm as f64)
        .sum::<f64>()
        / measure_count.max(1) as f64;

    for measure in grouped_measures.values_mut() {
        let weight = if average_npm > 0.0 {
            measure.measure.npm as f64 / average_npm
        } else {
            1.0
        };

        let pattern = {
            if measure.notes.windows(2).any(|w| {
                w[0].notes.iter().zip(w[1].notes.iter()).any(|(p, n)| *p && *n)
            }) {
                SecondaryPattern::Jack(determine_jack_type(measure))
            } else if measure.notes.iter().any(|n| matches!(n.pattern, BasePattern::Hand)) {
                SecondaryPattern::Handstream(determine_hs_type(measure))
            } else if measure.notes.iter().any(|n| matches!(n.pattern, BasePattern::Jump)) {
                SecondaryPattern::Jumpstream(determine_js_type(measure))
            } else if measure.notes.iter().any(|n| matches!(n.pattern, BasePattern::Single)) {
                SecondaryPattern::Singlestream(SinglestreamPattern::Singlestream)
            } else {
                SecondaryPattern::None
            }
        };


        measure.secondary_pattern = pattern.clone();
        *pattern_counts.entry(pattern).or_insert(0.0) += weight;

    }

    pattern_counts
}

fn determine_jack_type(measure: &mut ManiaMeasure) -> JackPattern {
    let mut pattern_count: HashMap<BasePattern, usize> = HashMap::new();

    for note in measure.notes() {
        *pattern_count.entry(note.pattern.clone()).or_insert(0) += 1;
    }

    let single = *pattern_count.get(&BasePattern::Single).unwrap_or(&0);
    let jump = *pattern_count.get(&BasePattern::Jump).unwrap_or(&0);
    let hand = *pattern_count.get(&BasePattern::Hand).unwrap_or(&0);
    let quad = *pattern_count.get(&BasePattern::Quad).unwrap_or(&0);

    if hand > jump + single {
        JackPattern::DenseChordjack
    } else if quad > 0 && jump + hand + quad > single {
        JackPattern::Chordjack
    } else {
        determine_jackspeed_or_chordstream(measure)
    }
}

fn determine_jackspeed_or_chordstream(measure: &mut ManiaMeasure) -> JackPattern {
    let mut jack_count = 0;
    for (i, note) in measure.notes.iter().enumerate() {
        if i > 0 {
            let prev = &measure.notes[i - 1];
            if note.notes.iter().zip(prev.notes.iter()).any(|(n, p)| *n && *p) {
                jack_count += 1;
            }
        }
    }
    if jack_count <= 1 && measure.tNotes() > 6 {
        JackPattern::ChordStream
    } else {
        JackPattern::Speedjack
    }
}

fn determine_hs_type(measure: &mut ManiaMeasure) -> HandstreamPattern {
    let mut pattern_count: HashMap<BasePattern, usize> = HashMap::new();

    for note in measure.notes() {
        *pattern_count.entry(note.pattern.clone()).or_insert(0) += 1;
    }
    let jump = *pattern_count.get(&BasePattern::Jump).unwrap_or(&0);

    if jump == 0 {
        HandstreamPattern::LightHs
    } else if jump > 0 {
        HandstreamPattern::DenseHs
    } else {
        HandstreamPattern::HS
    }
}
fn determine_js_type(measure: &mut ManiaMeasure) -> JumpstreamPattern {
    // Vérifie d'abord s'il y a deux sauts consécutifs
    if has_two_consecutive_jumps(measure) {
        return JumpstreamPattern::JT;
    }

    // Compte les différents types de patterns
    let mut pattern_count: HashMap<BasePattern, usize> = HashMap::new();
    for note in measure.notes() {
        *pattern_count.entry(note.pattern.clone()).or_insert(0) += 1;
    }
    let single = *pattern_count.get(&BasePattern::Single).unwrap_or(&0);
    let jump = *pattern_count.get(&BasePattern::Jump).unwrap_or(&0);

    // Crée un vecteur pour compter les notes actives par colonne
    let mut vect_int = vec![0; measure.notes[0].notes.len()];

    // Compte combien de fois chaque colonne est utilisée
    for note in measure.notes.iter() {
        for (i, &is_active) in note.notes.iter().enumerate() {
            if is_active {
                vect_int[i] += 1;
            }
        }
    }

    // Détermine le type de jumpstream basé sur les statistiques
    if let Some(&max_value) = vect_int.iter().max() {
        if max_value > 3 {
            JumpstreamPattern::AnchorJs
        } else if jump < single {
            JumpstreamPattern::LightJs
        } else {
            JumpstreamPattern::JS
        }
    } else {
        JumpstreamPattern::JS
    }
}

// Fonction helper pour vérifier les sauts consécutifs
fn has_two_consecutive_jumps(measure: &ManiaMeasure) -> bool {
    let mut last_was_jump = false;

    for note in measure.notes.iter() {
        let is_jump = matches!(note.pattern, BasePattern::Jump);

        if is_jump && last_was_jump {
            return true;
        }

        last_was_jump = is_jump;
    }

    false
}