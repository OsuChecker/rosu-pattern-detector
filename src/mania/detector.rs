use crate::mania::models::base::ManiaMeasure;
use crate::mania::models::base::{NotesStruct, Notes};
use crate::mania::models::pattern::{Pattern, JackPattern, HandstreamPattern, JumpstreamPattern, SinglestreamPattern};
use std::collections::{BTreeMap, HashMap};



pub(crate) fn analyze_patterns_by_measures_advanced(
    grouped_measures: &mut BTreeMap<i32, ManiaMeasure>,
) -> HashMap<Pattern, f64> {
    
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
                w[0].get_pattern() == Notes::Single && w[1].get_pattern() == Notes::Jump
            }) {
                Pattern::Jack(JackPattern::determine_jack_type(measure))
            } else if measure.notes.iter().any(|n| n.get_pattern() == Notes::Hand) {
                Pattern::Handstream(HandstreamPattern::determine_hs_type(measure))
            } else if measure.notes.iter().any(|n| n.get_pattern() == Notes::Jump) {
                Pattern::Jumpstream(JumpstreamPattern::determine_js_type(measure))
            } else if measure.notes.iter().any(|n| n.get_pattern() == Notes::Single) {
                Pattern::Singlestream(SinglestreamPattern::Singlestream)
            } else {
                Pattern::None
            }
        };


        measure.pattern = pattern.clone();
        *pattern_counts.entry(pattern).or_insert(0.0) += weight;

    }

    pattern_counts
}
