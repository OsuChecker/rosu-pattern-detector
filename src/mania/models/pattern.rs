use crate::mania::models::base::{Notes, ManiaMeasure};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone,Hash,Eq)]
pub enum Pattern
{
    Jack(JackPattern),
    Handstream(HandstreamPattern),
    Jumpstream(JumpstreamPattern),
    Singlestream(SinglestreamPattern),
    None,
} 

impl Pattern {
    pub fn to_all(&self) -> Pattern {
        match self {
            Pattern::Jack(_) => Pattern::Jack(JackPattern::All),
            Pattern::Handstream(_) => Pattern::Handstream(HandstreamPattern::All),
            Pattern::Jumpstream(_) => Pattern::Jumpstream(JumpstreamPattern::All),
            Pattern::Singlestream(_) => Pattern::Singlestream(SinglestreamPattern::All),
            Pattern::None => Pattern::None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum JackPattern {
    Chordjack,
    DenseChordjack,
    ChordStream,
    Speedjack,
    All,
}
impl JackPattern {
    pub fn determine_jack_type(measure: &mut ManiaMeasure) -> JackPattern {
        let mut pattern_count: HashMap<Notes, usize> = HashMap::new();
    
        for note in measure.notes.iter() {
            *pattern_count.entry(note.get_pattern()).or_insert(0) += 1;
        }
    
        let single = *pattern_count.get(&Notes::Single).unwrap_or(&0);
        let jump = *pattern_count.get(&Notes::Jump).unwrap_or(&0);
        let hand = *pattern_count.get(&Notes::Hand).unwrap_or(&0);
        let quad = *pattern_count.get(&Notes::Quad).unwrap_or(&0);
    
        if hand > jump + single {
            JackPattern::DenseChordjack
        } 
        else if quad > 0 && jump + hand > single 
        {
            JackPattern::Chordjack
        } 
        else
        {
            JackPattern::determine_jackspeed_or_chordstream(measure)
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
        if jack_count <= 1 && measure.notes.len() > 6 {
            JackPattern::ChordStream
        } else {
            JackPattern::Speedjack
        }
    }
}


#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum JumpstreamPattern {
    LightJs,
    AnchorJs,
    JS,
    JT,
    All,
}
impl JumpstreamPattern {
    fn has_two_consecutive_jumps(measure: &ManiaMeasure) -> bool {
        let mut last_was_jump = false;
    
        for note in measure.notes.iter() {
            let is_jump = matches!(note.get_pattern(), Notes::Jump);
    
            if is_jump && last_was_jump {
                return true;
            }
    
            last_was_jump = is_jump;
        }
    
        false
    }

    pub fn determine_js_type(measure: &mut ManiaMeasure) -> JumpstreamPattern {
        // Compte les différents types de patterns
        if JumpstreamPattern::has_two_consecutive_jumps(measure) {
            return JumpstreamPattern::JT;
        }
        
        let mut pattern_count: HashMap<Notes, usize> = HashMap::new();
        for note in measure.notes.iter() {
            *pattern_count.entry(note.get_pattern()).or_insert(0) += 1;
        }
        let single = *pattern_count.get(&Notes::Single).unwrap_or(&0);
        let jump = *pattern_count.get(&Notes::Jump).unwrap_or(&0);
    
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
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum HandstreamPattern {
    LightHs,
    AnchorHs,
    DenseHs,
    HS,
    All,
}
impl HandstreamPattern {
    pub fn determine_hs_type(measure: &mut ManiaMeasure) -> HandstreamPattern {
        let mut pattern_count: HashMap<Notes, usize> = HashMap::new();
    
        for note in measure.notes.iter() {
            *pattern_count.entry(note.get_pattern()).or_insert(0) += 1;
        }
        let jump = *pattern_count.get(&Notes::Jump).unwrap_or(&0);
    
        if jump == 0 {
            HandstreamPattern::LightHs
        } else if jump > 0 {
            HandstreamPattern::DenseHs
        } else {
            HandstreamPattern::HS
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum SinglestreamPattern {
    Singlestream,
    All,
}