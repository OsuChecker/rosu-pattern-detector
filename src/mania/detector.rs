use crate::mania::models::base::ManiaMeasure;
use crate::mania::models::pattern::Pattern;
use std::collections::HashMap;

// needed for the analyzer in order to do calc on it
// i32 is the timestamp of the start of the measure
// ManiaMeasure is the measure of the hit object
pub struct HitObjects(pub HashMap<i32, ManiaMeasure>);

pub struct PatternsValues(HashMap<Pattern, f64>);

impl std::fmt::Display for PatternsValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PatternsValues {{")?;
        for (pattern, value) in self.0.iter() {
            write!(f, "{}: {}, ", pattern, value)?;
        }
        write!(f, "}}")
    }
}
impl PatternsValues {
    fn add_pattern(&mut self, pattern: Pattern, value: f64) {
        *self.0.entry(pattern).or_insert(0.0) += value;
    }
    
    pub fn ordered_print(&self) {
        let mut sorted: Vec<(Pattern, f64)> = self.0.iter()
            .map(|(pattern, &value)| (pattern.clone(), value))
            .collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        println!("Patterns (sorted by value):");
        for (pattern, value) in sorted {
            println!("  {:?}: {:.2}", pattern, value);
        }
    }
}

impl HitObjects {
    pub fn get_npm(&self) -> f64 {
        self.0.values().map(|measure| measure.measure.npm as f64).sum::<f64>() / self.0.len() as f64
    }

    pub fn get_patterns_values(&self) -> PatternsValues {
        let mut patterns = PatternsValues(HashMap::new());
        for (_, measure) in self.0.iter() {
            patterns.add_pattern(measure.pattern.clone(), measure.value);        
        }
        patterns
    }
}


pub(crate) fn analyze_patterns(hit_objects: &mut HitObjects)
{
    let average_npm = hit_objects.get_npm();

    for (_, measure) in hit_objects.0.iter_mut() {
        measure.pattern = measure.detect_pattern();
        measure.value = measure.get_pattern_weight_modifier(average_npm);
    }
}
