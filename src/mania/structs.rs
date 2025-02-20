use crate::structs::CommonMeasure;

#[derive(Debug, Clone)]
pub struct Notes {
    pub(crate) timestamp: i32,
    pub(crate) notes: Vec<bool>,
    pub(crate) pattern: BasePattern,
}
impl Notes {
    pub fn to_display_string(&self) -> String {
        self.notes
            .iter()
            .map(|&active| if active { 'O' } else { 'X' })
            .collect()
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BasePattern {
    Single,
    Jump,
    Hand,
    Quad,
    Chord,
    None
}

#[derive(Debug, PartialEq, Clone,Hash,Eq)]
pub enum SecondaryPattern
{
    Jack(JackPattern),
    Handstream(HandstreamPattern),
    Jumpstream(JumpstreamPattern),
    Singlestream(SinglestreamPattern),
    None,
}
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum JackPattern {
    Chordjack,
    DenseChordjack,
    ChordStream,
    Speedjack,
    All,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum JumpstreamPattern {
    LightJs,
    AnchorJs,
    JS,
    JT,
    All,
}
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum HandstreamPattern {
    LightHs,
    AnchorHs,
    DenseHs,
    HS,
    All,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum SinglestreamPattern {
    Singlestream,
    All,
}

impl SecondaryPattern {
    pub fn to_all(&self) -> SecondaryPattern {
        match self {
            SecondaryPattern::Jack(_) => SecondaryPattern::Jack(JackPattern::All),
            SecondaryPattern::Handstream(_) => SecondaryPattern::Handstream(HandstreamPattern::All),
            SecondaryPattern::Jumpstream(_) => SecondaryPattern::Jumpstream(JumpstreamPattern::All),
            SecondaryPattern::Singlestream(_) => SecondaryPattern::Singlestream(SinglestreamPattern::All),
            SecondaryPattern::None => SecondaryPattern::None,
        }
    }
}


#[derive(Debug)]
pub struct ManiaMeasure {
    pub(crate) measure: CommonMeasure,
    pub(crate) notes: Vec<Notes>,
    pub(crate) secondary_pattern: SecondaryPattern,
}

impl ManiaMeasure {
    pub(crate) fn notes(&self) -> &Vec<Notes> {
        &self.notes
    }

    pub fn print_notes(&self) {
        for note in &self.notes {
            let line = note.to_display_string();
            println!("{}", line);
        }
    }

    pub fn tNotes(&self) -> i32 {
        self.notes
            .iter()
            .flat_map(|v| v.notes.iter())
            .filter(|&&b| b)
            .count() as i32
    }

    // Helper pour accéder aux détails spécifiques si nécessaire
    pub fn get_jack_pattern(&self) -> Option<&JackPattern> {
        match &self.secondary_pattern {
            SecondaryPattern::Jack(pattern) => Some(pattern),
            _ => None,
        }
    }
}


