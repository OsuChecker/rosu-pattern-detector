use std::fmt;
use serde::ser::{Serialize, Serializer};

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
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SecondaryPattern
{
    Jack,
    Handstream,
    Jumpstream,
    Singlestream,
    None,
}
impl fmt::Display for SecondaryPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SecondaryPattern::Jumpstream => write!(f, "JS"),
            SecondaryPattern::Jack => write!(f, "Jack"),
            SecondaryPattern::Handstream => write!(f, "HS"),
            SecondaryPattern::Singlestream => write!(f, "SS"),
            SecondaryPattern::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Ord, Eq, Clone, PartialOrd)]
pub enum TertiaryPattern
{
    DENSE_CHORDJACK,
    CHORDJACK,
    SPEEDJACK,
    CHORDSTREAM,
    LIGHT_JS,
    ANCHOR_JS,
    JS,
    JT,
    LIGHT_HS,
    ANCHOR_HS,
    DENSE_HS,
    HS,
    None,
}
impl fmt::Display for TertiaryPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TertiaryPattern::DENSE_CHORDJACK => write!(f, "Dense Chordjack"),
            TertiaryPattern::CHORDJACK => write!(f, "Chordjack"),
            TertiaryPattern::SPEEDJACK => write!(f, "Speedjack"),
            TertiaryPattern::CHORDSTREAM => write!(f, "ChordStream"),
            TertiaryPattern::LIGHT_JS => write!(f, "Light JS"),
            TertiaryPattern::ANCHOR_JS => write!(f, "Anchor JS"),
            TertiaryPattern::JS => write!(f, "JS"),
            TertiaryPattern::JT => write!(f, "JT"),
            TertiaryPattern::ANCHOR_HS => write!(f, "Anchor HS"),
            TertiaryPattern::LIGHT_HS => write!(f, "Light HS"),
            TertiaryPattern::DENSE_HS => write!(f, "Dense HS"),
            TertiaryPattern::HS => write!(f, "HS"),
            TertiaryPattern::None => write!(f, "None"),
        }
    }
}
impl Serialize for TertiaryPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug)]
pub struct ManiaMeasure {
    pub(crate) measure: crate::structs::CommonMeasure,
    pub(crate) notes: Vec<Notes>,
    pub(crate) secondary_pattern: SecondaryPattern,
    pub(crate) tertiary_pattern: TertiaryPattern,
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

    pub fn tNotes(&self) -> i32
    {
        self.notes
            .iter()
            .flat_map(|v| v.notes.iter())
            .filter(|&&b| b)
            .count() as i32
    }

}


